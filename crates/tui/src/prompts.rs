// TODO(integrate): Move prompt building from engine into this module — tracked as future refactoring
#![allow(dead_code)]

//! System prompts for different modes.
//! NOTE: Prompt building is currently handled directly in engine - these are for future refactoring.

use crate::models::SystemPrompt;
use crate::project_context::{ProjectContext, load_project_context_with_parents};
use crate::tui::app::AppMode;
use std::path::Path;

/// Conventional location for the structured session-handoff artifact (#32).
/// A previous session writes it on exit / `/compact`; the next session reads
/// it back on startup and prepends it to the system prompt so a fresh agent
/// doesn't have to re-discover open blockers from scratch.
pub const HANDOFF_RELATIVE_PATH: &str = ".deepseek/handoff.md";

/// Read the workspace-local handoff artifact, if present, and format it as a
/// system-prompt block. Returns `None` when the file is absent or empty so
/// callers can keep the default-uncluttered prompt for fresh workspaces.
fn load_handoff_block(workspace: &Path) -> Option<String> {
    let path = workspace.join(HANDOFF_RELATIVE_PATH);
    let raw = std::fs::read_to_string(&path).ok()?;
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    Some(format!(
        "## Previous Session Handoff\n\nThe previous session in this workspace left a handoff at `{}`. Consider it the first artifact to read on this turn — open blockers, in-flight changes, and recent decisions live there. Update or rewrite it before exiting if state changes materially.\n\n{}",
        HANDOFF_RELATIVE_PATH, trimmed
    ))
}

// Prompt files loaded at compile time
pub const BASE_PROMPT: &str = include_str!("prompts/base.txt");
#[allow(dead_code)]
pub const NORMAL_PROMPT: &str = include_str!("prompts/normal.txt");
pub const AGENT_PROMPT: &str = include_str!("prompts/agent.txt");
pub const YOLO_PROMPT: &str = include_str!("prompts/yolo.txt");
pub const PLAN_PROMPT: &str = include_str!("prompts/plan.txt");

fn mode_prompt(mode: AppMode) -> &'static str {
    match mode {
        AppMode::Agent => AGENT_PROMPT,
        AppMode::Yolo => YOLO_PROMPT,
        AppMode::Plan => PLAN_PROMPT,
    }
}

fn compose_mode_prompt(mode: AppMode) -> String {
    format!("{}\n\n{}", BASE_PROMPT.trim(), mode_prompt(mode).trim())
}

/// Get the system prompt for a specific mode
pub fn system_prompt_for_mode(mode: AppMode) -> SystemPrompt {
    SystemPrompt::Text(compose_mode_prompt(mode))
}

/// Get the system prompt for a specific mode with project context
pub fn system_prompt_for_mode_with_context(
    mode: AppMode,
    workspace: &Path,
    working_set_summary: Option<&str>,
) -> SystemPrompt {
    let mode_prompt = compose_mode_prompt(mode);

    // Load project context from workspace
    let project_context = load_project_context_with_parents(workspace);

    // Combine base prompt with project context
    let mut full_prompt = if let Some(project_block) = project_context.as_system_block() {
        format!("{}\n\n{}", mode_prompt, project_block)
    } else {
        // Fallback: Generate an automatic project map summary
        let summary = crate::utils::summarize_project(workspace);
        let tree = crate::utils::project_tree(workspace, 2); // Shallow tree for prompt
        format!(
            "{}\n\n### Project Structure (Automatic Map)\n**Summary:** {}\n\n**Tree:**\n```\n{}\n```",
            mode_prompt, summary, tree
        )
    };

    if let Some(summary) = working_set_summary
        && !summary.trim().is_empty()
    {
        full_prompt = format!("{full_prompt}\n\n{summary}");
    }

    if let Some(handoff_block) = load_handoff_block(workspace) {
        full_prompt = format!("{full_prompt}\n\n{handoff_block}");
    }

    // Add compaction instruction for agent modes
    if matches!(mode, AppMode::Agent | AppMode::Yolo) {
        full_prompt.push_str(
            "\n\n## Context Management\n\n\
             When the conversation gets long (you'll see a context usage indicator), you can:\n\
             1. Use `/compact` to summarize earlier context and free up space\n\
             2. The system will preserve important information (files you're working on, recent messages, tool results)\n\
             3. After compaction, you'll see a summary of what was discussed and can continue seamlessly\n\n\
             If you notice context is getting long (>80%), proactively suggest using `/compact` to the user."
        );
    }

    SystemPrompt::Text(full_prompt)
}

/// Build a system prompt with explicit project context
pub fn build_system_prompt(base: &str, project_context: Option<&ProjectContext>) -> SystemPrompt {
    let full_prompt =
        match project_context.and_then(super::project_context::ProjectContext::as_system_block) {
            Some(project_block) => format!("{}\n\n{}", base.trim(), project_block),
            None => base.trim().to_string(),
        };
    SystemPrompt::Text(full_prompt)
}

// Legacy functions for backwards compatibility
pub fn base_system_prompt() -> SystemPrompt {
    SystemPrompt::Text(BASE_PROMPT.trim().to_string())
}

pub fn normal_system_prompt() -> SystemPrompt {
    system_prompt_for_mode(AppMode::Agent)
}

pub fn agent_system_prompt() -> SystemPrompt {
    system_prompt_for_mode(AppMode::Agent)
}

pub fn yolo_system_prompt() -> SystemPrompt {
    system_prompt_for_mode(AppMode::Yolo)
}

pub fn plan_system_prompt() -> SystemPrompt {
    system_prompt_for_mode(AppMode::Plan)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn plan_prompt_prefers_best_effort_plans_over_clarifying_loops() {
        let prompt = match system_prompt_for_mode(AppMode::Plan) {
            SystemPrompt::Text(text) => text,
            SystemPrompt::Blocks(_) => panic!("expected text system prompt"),
        };

        assert!(prompt.contains("Default to publishing a best-effort plan immediately."));
        assert!(prompt.contains("your first action should be update_plan."));
        assert!(prompt.contains("do not browse the repo first"));
        assert!(prompt.contains("Do not ask clarifying questions for straightforward requests"));
        assert!(prompt.contains("If the user asks for \"a 3-step plan\""));
    }

    /// Discriminator unique to the injected handoff block (not present in the
    /// agent prompt's own discussion of the convention).
    const HANDOFF_BLOCK_MARKER: &str = "left a handoff at `.deepseek/handoff.md`";

    #[test]
    fn handoff_artifact_is_prepended_to_system_prompt_when_present() {
        let tmp = tempdir().expect("tempdir");
        let workspace = tmp.path();
        let handoff_dir = workspace.join(".deepseek");
        std::fs::create_dir_all(&handoff_dir).unwrap();
        std::fs::write(
            handoff_dir.join("handoff.md"),
            "# Session handoff — prior\n\n## Active task\nFinish #32.\n\n## Open blockers\n- [ ] write the basic version\n",
        )
        .unwrap();

        let prompt = match system_prompt_for_mode_with_context(AppMode::Agent, workspace, None) {
            SystemPrompt::Text(text) => text,
            SystemPrompt::Blocks(_) => panic!("expected text system prompt"),
        };

        assert!(prompt.contains(HANDOFF_BLOCK_MARKER));
        assert!(prompt.contains("Finish #32."));
        assert!(prompt.contains("write the basic version"));
    }

    #[test]
    fn missing_handoff_does_not_inject_block() {
        let tmp = tempdir().expect("tempdir");
        let prompt = match system_prompt_for_mode_with_context(AppMode::Agent, tmp.path(), None) {
            SystemPrompt::Text(text) => text,
            SystemPrompt::Blocks(_) => panic!("expected text system prompt"),
        };
        assert!(!prompt.contains(HANDOFF_BLOCK_MARKER));
    }

    #[test]
    fn empty_handoff_file_does_not_inject_block() {
        let tmp = tempdir().expect("tempdir");
        let dir = tmp.path().join(".deepseek");
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("handoff.md"), "   \n\n  ").unwrap();
        let prompt = match system_prompt_for_mode_with_context(AppMode::Agent, tmp.path(), None) {
            SystemPrompt::Text(text) => text,
            SystemPrompt::Blocks(_) => panic!("expected text system prompt"),
        };
        assert!(!prompt.contains(HANDOFF_BLOCK_MARKER));
    }
}

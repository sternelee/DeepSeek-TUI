# Modes and Approvals

DeepSeek TUI has two related concepts:

- **TUI mode**: what kind of visible interaction you're in (Plan/Agent/YOLO/Hetun).
- **Approval mode**: how aggressively the UI asks before executing tools.

## TUI Modes

Press `Tab` to cycle through the visible modes: **Plan → Agent → YOLO → Hetun → Plan**.
Press `Shift+Tab` to cycle in reverse. Hetun sits at the end of the cycle so a fresh session doesn't land on it accidentally — the default landing mode is unchanged.

- **Plan**: design-first prompting. Read-only investigation tools stay available; shell and patch execution stay off. Use this when you want to think out loud and produce a plan to hand to a human (yourself later, or a reviewer).
- **Agent**: multi-step tool use. Approvals for shell and paid tools (file writes are allowed without a prompt). RLM is available — the model reaches for `repl` blocks when the work is decomposable.
- **YOLO**: enables shell + trust mode and auto-approves all tools. RLM is available and auto-executes like everything else. Use only in trusted repos.
- **Hetun** (河豚, "Plan + Recursive Agents"): the most opinionated mode the TUI offers. The model uses RLM aggressively to research and decompose tasks in parallel via cheap `deepseek-v4-flash` child calls, then presents a consolidated **mission** for your approval. Once approved, the RLM tree auto-executes without per-tool interruption — you approve the mission, not each individual bullet. Plan + execution folded into one rhythm.

## Compatibility Notes

- `/normal` is a hidden compatibility alias that switches to `Agent`.
- Older settings files with `default_mode = "normal"` still load as `agent`; saving rewrites the normalized value.

## Escape Key Behavior

`Esc` is a cancel stack, not a mode switch.

- Close slash menus or transient UI first.
- Cancel the active request if a turn is running.
- Discard a queued draft if the composer is empty.
- Clear the current input if text is present.
- Otherwise it is a no-op.

## Approval Mode

You can override approval behavior at runtime:

```text
/config
# edit the approval_mode row to: suggest | auto | never
```

Legacy note: `/set approval_mode ...` was retired in favor of `/config`.

- `suggest` (default): uses the per-mode rules above.
- `auto`: auto-approves all tools (similar to YOLO approval behavior, but without forcing YOLO mode).
- `never`: blocks any tool that isn't considered safe/read-only.

### Task-level approval (Hetun mode)

Hetun mode introduces a higher-level approval concept. Before executing an RLM tree, the engine presents a **mission card** showing what will be done, estimated flash calls, and expected outcomes. You can:

- **Approve** — the RLM tree runs without further prompts.
- **Reject** — the engine returns to planning.
- **Modify** — edit the mission description and re-submit.

This is independent of the base `approval_mode` setting. If you set `approval_mode = auto` while in Hetun, you still see mission cards (task-level approval is part of the mode, not the approval policy).

## Small-Screen Status Behavior

When terminal height is constrained, the status area compacts first so header/chat/composer/footer remain visible:

- Loading and queued status rows are budgeted by available height.
- Queued previews collapse to compact summaries when full previews do not fit.
- `/queue` workflows remain available; compact status only affects rendering density.

## Workspace Boundary and Trust Mode

By default, file tools are restricted to the `--workspace` directory. Enable trust mode to allow file access outside the workspace:

```text
/trust
```

YOLO mode enables trust mode automatically.

## MCP Behavior

MCP tools are exposed as `mcp_<server>_<tool>` and use the same approval flow as built-in tools. Read-only MCP helpers may auto-run in suggestive approval modes; MCP tools with possible side effects require approval.

See `MCP.md`.

## Related CLI Flags

Run `deepseek --help` for the canonical list. Common flags:

- `-p, --prompt <TEXT>`: one-shot prompt mode (prints and exits)
- `--model <MODEL>`: when using the `deepseek` facade, forward a DeepSeek model override to the TUI
- `--workspace <DIR>`: workspace root for file tools
- `--yolo`: start in YOLO mode
- `--hetun`: start in Hetun mode
- `-r, --resume <ID|PREFIX|latest>`: resume a saved session
- `-c, --continue`: resume the most recent session
- `--max-subagents <N>`: clamp to `1..=20`
- `--no-alt-screen`: run inline without the alternate screen buffer
- `--mouse-capture` / `--no-mouse-capture`: opt in or out of internal mouse scrolling/selection. Mouse capture is enabled by default when the alternate screen is active; use `--no-mouse-capture` when you need terminal-native drag selection.
- `--profile <NAME>`: select config profile
- `--config <PATH>`: config file path
- `-v, --verbose`: verbose logging

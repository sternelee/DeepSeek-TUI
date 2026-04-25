# RLM as a Fundamental Agent Primitive

## Thesis

We will make Recursive Language Models a first-class primitive in `deepseek-tui` by teaching the flat agent loop to detect fenced ```` ```repl ```` blocks in assistant text and hand them directly to the external `zigrlm` binary. `zigrlm` orchestrates cheap parallel `deepseek-v4-flash` child calls, runs a JS sandbox, and returns a single `FINAL` result that becomes the assistant's response for that turn. This replaces the heavy `agent_swarm` tokio-task-per-child model with a lightweight subprocess tree where N flash calls cost less than one Pro call, inverting the usual sub-agent economics.

## Where We Are Today

The agent loop is in `crates/tui/src/core/engine.rs` (`Engine::handle_deepseek_turn()`, ~line 2330). It streams `ContentBlock`s from the model into `session.messages`; if any block is `ToolUse`, it builds a `ToolExecutionPlan`, executes via `execute_tool_with_lock()` (~line 2209), and loops back for another model turn. Parallel work today goes through `AgentSwarmTool` in `crates/tui/src/tools/swarm.rs` (`run_swarm()`, ~line 582), which spawns full background tokio tasks via `SubAgentManager::spawn_background_with_assignment()` in `crates/tui/src/tools/subagent.rs` (~line 584). Each child runs its own agent loop, tool registry, and event channel. That is correct for autonomous multi-step work, but wasteful for simple parallel Q&A or recursive decomposition.

`zigrlm` (already built at `/Volumes/VIXinSSD/zigrlm/zig-out/bin/zigrlm` or cloneable from GitHub) solves this externally. Its `cli` command reads a prompt, drives a root model turn, parses any ```` ```repl ```` blocks with its Zig-native parser (`src/parser.zig`), fans out batched child calls across OS threads capped by `max_concurrent_subcalls` (default 8), and returns the `FINAL` string on stdout. The integration work is wiring this into the engine so the model naturally emits repl blocks instead of JSON tool calls.

## Key Design Questions

### 1. How is `zigrlm` auto-configured with DeepSeek credentials? (#48)

**New file:** `crates/tui/src/zigrlm_config.rs`

A `ZigrlmRuntimeConfig` struct is built from the session's existing `ResolvedRuntimeOptions` (`api_key`, `base_url`, `model`). It constructs the two environment variables `zigrlm` expects:

- `ZIGRLM_MAIN_CMD`: `zigrlm openai-proxy --model <pro> --base-url <url>`
- `ZIGRLM_RLM_CMD`: `zigrlm openai-proxy --model deepseek-v4-flash --base-url <url>`

The API key is passed as `OPENAI_API_KEY`. Binary discovery (in priority order): `config.toml` field `zigrlm.bin_path` → env `ZIGRLM_BIN` → known local build `/Volumes/VIXinSSD/zigrlm/zig-out/bin/zigrlm` → `PATH` via `which zigrlm`. If not found, RLM features degrade gracefully with a logged warning.

**Config additions:** `crates/config/src/lib.rs` gets a `ZigrlmConfigToml` struct with optional overrides for `bin_path`, `rlm_model`, `max_depth` (default 2), `max_iterations` (default 20), and `timeout_ms` (default 600000). These are exposed under a new `zigrlm:` table in `config.toml`.

### 2. How does the engine detect and execute repl blocks? (#49)

**Modified file:** `crates/tui/src/core/engine.rs`

After the streaming loop in `handle_deepseek_turn()` persists the assistant message to `session.messages`, we insert a new branch before tool execution:

```rust
if message.has_tool_calls() {
    // existing tool-execution path
} else if has_repl_block(&message.content) {
    let result = zigrlm_runtime.run_inline(&message.content).await?;
    // Replace the Text block with the aggregated result
    message.replace_repl_with_result(&result.response);
    // Append usage metadata as a system note or hidden block
    if let Some(usage) = result.usage {
        session.add_system_note(format!(
            "[RLM: {} calls, {} tokens]", usage.calls, usage.total_tokens
        ));
    }
    // Turn completes; no extra model round-trip
}
```

`has_repl_block()` checks `ContentBlock::Text` for the exact substring "\`\`\`repl" using the same fence logic as `zigrlm/src/parser.zig`. `run_inline()` lives in the new `crates/tui/src/zigrlm_runtime.rs` and shells out to:

```bash
zigrlm cli \
  --max-depth 2 \
  --max-iterations 20 \
  --timeout-ms 600000 \
  "<assistant_text>"
```

with `ZIGRLM_MAIN_CMD`, `ZIGRLM_RLM_CMD`, and `OPENAI_API_KEY` injected into the child environment. The full assistant text is the prompt because the model's natural-language plan preceding the fence is part of the root context `zigrlm` expects.

**UX:** While `zigrlm` runs, the engine emits `Event::RlmStarted` and the TUI shows a spinner: "Running RLM tree…". On completion, `Event::RlmComplete` carries usage so the transcript can render a collapsible "[RLM: 3 calls, 2.1K tokens, 1.2s]" line. `Ctrl-C` during this phase forwards `SIGTERM` to the child process.

### 3. How does the result re-enter the conversation?

The raw assistant message is mutated in-place in `session.messages` (`crates/tui/src/core/session.rs`). Its `ContentBlock::Text` block containing the repl fence is replaced by the `FINAL` string from `zigrlm` stdout. The original repl block is preserved as a `ContentBlock::Thinking` block (or a new internal metadata field) so the model can see its own plan on subsequent turns, but the primary visible response is the aggregated result. This keeps the conversation history clean: the next turn's context contains the unified answer, not raw DSL.

### 4. What happens to the explicit `zigrlm` tool / bridge? (#46)

It remains as an **escape hatch** in `crates/tui/src/tools/zigrlm.rs` (new file), registered via `ToolRegistryBuilder::with_zigrlm_tool()` in `crates/tui/src/tools/registry.rs`. The tool accepts explicit parameters (`prompt`, `max_depth`, `trace_path`, etc.) and is useful for:

- DSPy-style signatures via `dszig`
- Docker-backed Python sandboxes
- Custom traces for benchmarking
- User-explicit RLM experiments

The inline primitive and the explicit tool share `ZigrlmRuntimeConfig` but serve different purposes. The model prompt (see below) teaches when to use each.

### 5. How do we teach the model to use this? (#50)

**Modified files:** `crates/tui/src/prompts/agent.txt`, `crates/tui/src/prompts/yolo.txt`

A new section, gated by config flag `rlm.prompt_enabled` (default `true`), is appended to the agent system prompt:

```text
## Recursive Language Model (RLM) primitive

When you need parallel analysis, recursive decomposition, or batched generation,
prefer a fenced `repl` block over spawning subagents or doing sequential inline work.

- `rlm_query_batched name = "prompt" | "prompt" | ...` for parallel work
- `rlm_query name = "prompt"` for recursive child tasks
- End with `FINAL(expression)` or `FINAL_VAR(name)`

The child model is deepseek-v4-flash (very fast and cheap).

Do NOT use RLM when the task requires file-system modification, interactive user
input, or is trivial enough for a single sentence.
```

A comparison table in the prompt clarifies the trade-offs:

| Primitive | Use when | Cost | Speed |
|---|---|---|---|
| Inline reasoning | Simple Q&A, one-step tasks | Low | Fast |
| `repl` block | Parallel / recursive / batched work | Very low (flash) | Fast |
| `agent_swarm` | Multi-step autonomous work with tools | Higher | Slower (polling) |

This lets us A/B test by toggling `rlm.prompt_enabled` and measuring turns-per-task and token usage.

### 6. How does the model do non-trivial work *inside* a `repl` block? (#53)

Parallel fan-out alone isn't enough. A `repl` block that just splits N prompts and concatenates results is barely better than `agent_swarm`. The unlock is giving the model **cheap programmatic access to data that's already in process memory** — so it can grep, extract, slice, diff, and search a 50K-token blob in one repl block without burning context tokens on the raw bytes or paying for a tool round-trip per query.

This is what makes RLM actually usable, not just clever. We bake a curated helper layer + a sandboxed Python REPL into the runtime as a first-class capability.

**The shape:**

A `repl` block doesn't only fan out to flash children. It can also run Python in a sandboxed namespace where:

- A `ctx` variable holds preloaded data the agent wants to interrogate (a file it just read, a tool result, a stream of search hits).
- A small curated helper module is in scope — about 15–25 functions chosen because they meaningfully beat shell when the data is already in memory: `peek` / `lines` / `head` / `tail` / `chunk` / `between`, `grep` / `count_matches` / `find_all` / `semantic_search`, `extract_json_objects` / `extract_urls` / `extract_paths` / `extract_dates`, `replace_all` / `split_by` / `diff` / `similarity`, `dedupe` / `group_by` / `partition` / `frequency`.
- Sandbox is AST-validated + restricted builtins + import allowlist + execution timeout — best-effort, same posture as the JS sandbox the runtime already exposes.
- State persists across `repl` blocks within a turn. The model can `let chunks = chunk(ctx, 4000)` once and reuse `chunks` in subsequent fan-outs.

**Why this lives at the runtime level, not as a separate tool:**

If we shipped a `python_repl` tool alongside RLM, the model would have to choose between "fan out to flash children" (repl block) and "inspect data in Python" (tool call) every turn. They're the same workflow — load → slice → fan out flash queries on the slices → aggregate. Splitting them across two interfaces forces the wrong choice. Putting the helper layer *inside* the repl runtime means a single block can do all four steps with shared state.

**Why these specific helpers and not a giant library:**

The model already has shell + grep + read_file. It doesn't need 124 helpers. It needs ~20 that are obviously the right move when working with in-memory data — the ones where shell would force an unnecessary round-trip or lose structure. Keep the menu small and obvious. Anything not on the menu, the model can write inline (Python is in the sandbox; helpers are conveniences, not a closed world).

## Spike Target

The smallest end-to-end proof is a hardcoded path in `crates/tui/src/core/engine.rs` that, when an assistant message contains a test repl block, shells out to a pre-built `zigrlm` binary with a hardcoded `ZIGRLM_RLM_CMD` pointing at `deepseek-v4-flash`, and injects the stdout result back into `session.messages`.

**Estimated surface area:**
- `engine.rs`: ~30 lines (detection branch + subprocess call)
- `zigrlm_runtime.rs` (new, spike version): ~80 lines (Command builder + stdout capture)
- No config plumbing, no TUI spinner, no usage parsing, no prompt changes.

**Success criteria:** A local test where the Pro model emits:

````
```repl
rlm_query_batched answers = "What is 2+2?" | "What is 3+3?"
FINAL_VAR(answers)
```
````

… and the engine returns a single assistant message containing the aggregated `[0]\n4\n[1]\n6` result, with no tool call JSON emitted and no extra model round-trip.

## Hetun Mode — "Plan + Recursive Agents" (added, doesn't replace Plan)

**Tracking issue:** #54

**Hetun** (河豚, Mandarin for *pufferfish*) is added as a fourth mode positioned at the **end** of the Tab cycle so people don't accidentally land on it from a fresh session. The cycle becomes `Plan → Agent → YOLO → Hetun → Plan`. Default landing mode is unchanged. Plan stays exactly as it is — read-only investigation, hand the plan to the human. Hetun is the next step further up the orchestration ladder: planning *and* execution folded together, gated on a single mission-level approval.

The mode badge surfaces this as **"Hetun (Plan + Recursive Agents)"** so users immediately understand the relationship to Plan. Sakana already named the flash-coordinator architecture *Fugu* (the Japanese reading of 河豚); since DeepSeek is Chinese the mandarin reading *hetun* is the right cultural fit.

### What Hetun does

It's the most opinionated mode the TUI offers. The model both **plans the work and runs it**, but the user gates the transition with one explicit mission approval:

1. **Research + plan** — Hetun uses RLM aggressively to investigate the workspace in parallel (multiple `rlm_query_batched` reads of relevant files / patterns / prior turns), synthesises the findings into a concrete mission (sub-tasks, what each looks at, expected outputs, anything that gets written), and lands it in the transcript ending with an explicit "OK to run?" prompt.
2. **Execute** — once approved, Hetun emits a `repl` block that fans the planned sub-tasks out via `rlm_query_batched` and aggregates into a `FINAL`. No further per-block approvals — you approved the **mission**, the runtime carries it out.

This is meaningfully different from Plan (read-only investigate, hand back to human, human implements) and from YOLO (auto-execute everything turn-by-turn). Hetun keeps the human in the loop at the only point that matters — the gate between "we know what to do" and "do it" — and removes them from every per-step approval after that.

### Behaviour and configuration

- **The user's configured model is left alone.** Entering Hetun does *not* swap the conversational model or reasoning effort. If you were on `deepseek-v4-pro` / `max`, you stay there. The flash-as-coordinator behaviour is internal to the runtime (`ZIGRLM_RLM_CMD` always points to flash regardless of mode), not a global model swap. On exit nothing has to be restored because nothing was changed.
- **No `/hetun` slash command.** Tab cycles into the mode like any other; `/plan` keeps switching to Plan as it does today.
- **Mission-level approval, not block-level.** Hetun introduces one approval gate per turn (the mission), then runs the execution `repl` block straight through. Inside Plan, Agent, and YOLO the existing approval policies are unchanged.

RLM is not Hetun-only. Agent and YOLO modes keep using `repl` blocks where the model judges them appropriate (#49 wires the inline primitive globally). Hetun is just the mode that *expects* RLM-first behaviour, and the prompt is tuned for it.

### What "Plan + Recursive Agents" actually means inside Hetun

Sakana's writeup of the Fugu / "intelligence" architecture (the system they shipped to MIC's misinformation programme) describes more than a flash-coordinator wrapper. The mode adopts those technical patterns and translates them into our primitives. A Hetun research phase is not one batched fan-out — it is a small recursive program that runs inside a `repl` block:

- **Novelty search via recursive sampling.** Instead of pre-deciding N fixed queries and firing them in parallel, Hetun draws an initial broad sample of the workspace (`ctx` chunks of relevant files / patterns / prior turns), runs a flash sweep over the sample asking "what here is surprising or important?", scores responses by novelty, and recursively zooms into the highest-novelty chunks at finer resolution. The recursion stops when novelty plateaus or a budget is hit. This gives much better coverage than a flat 8-way fan-out for any task where the interesting bits are non-uniformly distributed.
- **Hierarchical narrative tree synthesis.** Findings from the recursion don't get concatenated into a flat list. They get organised into a tree: leaves are individual observations, intermediate nodes are clusters of related findings, the root is the mission goal. The mission card the user approves displays this tree (collapsible, navigable) rather than a wall of bullets — same intuition Sakana uses to make SNS narrative spaces legible at a glance.
- **Multi-detector cross-verification.** Every claim Hetun puts into the mission goes through at least two passes: a flash sweep for obvious errors / contradictions, and a frontier (Pro) check for subtler structural issues. Sakana's framing is "frontier model handles macro structure, specialised models handle fine structure, blind spots cancel". For us that maps to flash for breadth and Pro for depth, with claims that fail either pass marked as low-confidence rather than hidden — the user can choose to verify them manually or drop them from the mission.
- **Hypothesis-verification loop.** The research phase isn't a single round. Hetun forms a working hypothesis from the initial findings, generates verification queries from the hypothesis (e.g. "if X is true, we should also see Y; check for Y"), runs them, and updates the hypothesis. The loop continues until the hypothesis is stable across iterations or the iteration budget (capped low — typically 2–3) is exhausted. This is the same hypothesis-driven investigation rhythm Sakana models on human fact-checkers.

These patterns are not separate features — they are how the Hetun prompt teaches the model to use `repl` blocks. The runtime primitives (`rlm_query_batched`, `ctx` helpers from #53, flash/Pro tiering from #48) are already in place once the rest of the RLM stack ships; Hetun is the prompt + approval layer that wires them together into the recursive-research-and-mission rhythm above.

We do **not** import Sakana's full system: the ABM persona-simulation framework (Shachi) and the misinformation-specific image/video detectors are out of scope. What we adopt is the **research methodology** — recursive novelty sampling, hierarchical synthesis, multi-detector verification, hypothesis loops — applied to the agent-coding domain instead of the misinformation domain.

**Files:** `crates/tui/src/tui/app.rs` (add `AppMode::Hetun`, place it last in the Tab cycle), `crates/tui/src/tui/palette.rs` (add `MODE_HETUN` colour, e.g. purple to distinguish from YOLO red and Plan orange), `crates/tui/src/tui/prompts.rs` (add `HETUN_PROMPT`), `crates/tui/src/prompts/hetun.txt` (the prompt body — must teach the recursive-novelty + hierarchical-synthesis + verification-loop pattern, not just "fan out queries"), `crates/tui/src/core/engine.rs` (mission-level approval hook before RLM execution in Hetun), `crates/tui/src/tui/widgets/header.rs` (mode-badge text reads "Hetun (Plan + Recursive Agents)").

## Vendoring zigrlm

**Tracking issue:** #55

Rather than treating `zigrlm` as an external binary, we will vendor it as a git submodule at `vendor/zigrlm` and build it alongside the Rust project. This lets us:

1. Guarantee the binary exists for contributors and CI
2. Patch zigrlm for deepseek-tui-specific features (three-tier model routing, custom JS builtins, DeepSeek trace format)
3. Eventually link it as a C library instead of shelling out

**Build integration:** A `build.rs` in `crates/tui/` invokes `zig build` in `vendor/zigrlm` when `zig` is on PATH. If `zig` is missing, the TUI falls back to existing binary-discovery logic. `ZigrlmRuntimeConfig` prefers the vendored path.

**Files:** `.gitmodules`, `crates/tui/build.rs`, `crates/tui/src/zigrlm_config.rs`, `README.md`, `AGENTS.md`.

## Plan

These ship together as one cohesive RLM landing — Hetun is the flagship that gives the rest a reason to exist on day one, the helper layer is what gives RLM something to do besides toy parallelism, and the auto-config + vendoring make it work without any user setup. The order below is the implementation dependency order, not a staggered release schedule. We just keep shipping.

| Issue | Scope | Files |
|---|---|---|
| #48 | **Auto-config.** Build `ZigrlmRuntimeConfig`, binary discovery, config schema. | `crates/tui/src/zigrlm_config.rs` (new), `crates/config/src/lib.rs` |
| #49 | **Inline primitive.** Detect repl blocks in `handle_deepseek_turn()`, shell out, replace message content, emit RLM events. | `crates/tui/src/core/engine.rs`, `crates/tui/src/zigrlm_runtime.rs` (new), `crates/tui/src/core/events.rs` |
| #53 | **Helper layer + Python sandbox.** Curated `ctx` helpers + AST-validated Python sandbox baked into the repl runtime. | New helper module (location TBD between zigrlm upstream and `crates/tui/src/zigrlm_runtime/`) |
| #55 | **Vendor zigrlm.** Add git submodule, build script, prefer vendored path. | `.gitmodules`, `crates/tui/build.rs`, `crates/tui/src/zigrlm_config.rs` |
| #50 | **Prompt engineering.** Add RLM section to agent/yolo prompts, config toggle, examples that exercise the helper layer. | `crates/tui/src/prompts/agent.txt`, `crates/tui/src/prompts/yolo.txt`, `crates/config/src/lib.rs` |
| #54 | **Hetun mode.** Add a 4th mode at the end of the cycle, with mission-level approval gate. Plan stays unchanged. | `crates/tui/src/tui/app.rs`, `crates/tui/src/tui/palette.rs`, `crates/tui/src/prompts/hetun.txt`, `crates/tui/src/core/engine.rs`, `crates/tui/src/tui/widgets/header.rs` |
| #46 | **Explicit bridge.** Implement `ZigrlmTool` spec, register in registry, add to sub-agent allowed lists. | `crates/tui/src/tools/zigrlm.rs` (new), `crates/tui/src/tools/registry.rs`, `crates/tui/src/tools/subagent.rs` |

We diverge from the old #40 plan (building a native Rust repl parser) because `zigrlm` already owns parsing, sandboxing, and trace emission. Reimplementing that in Rust is waste — #53 (the helper layer) is where we add the value that makes the runtime actually usable.

## Non-Goals (deferred)

- **Native repl parser in Rust** (#41–#45, all closed). zigrlm's Zig parser is sufficient.
- **Real-time streaming of child progress** into the TUI transcript. Spinner + final summary is enough.
- **Process pool / pre-warming** of zigrlm subprocesses. One fork per repl block is acceptable given flash latency.
- **Replacing `agent_swarm` entirely.** Swarm remains for multi-step autonomous work that requires tools.
- **Automatic migration** of existing swarm task graphs to repl blocks.
- **Windows-specific binary discovery quirks.** macOS / Linux are the priority surfaces.
- **JS sandbox hardening.** Trusted-local-compute model, same posture as the Python sandbox in #53.
- **Three-tier model routing** (frontier escalation inside repl). Requires zigrlm patches; do once vendoring (#52) lands.
- **Native C-library linkage** of zigrlm. Worth doing only after subprocess overhead is shown to be a real bottleneck.

---

## Appendix A: ReAct vs. RLM — Why Both?

> A deeper treatment lives in `docs/research-react-vs-rlm.md`. This appendix extracts the decisions that matter for our integration.

**ReAct** (Yao et al., ICLR 2023) is the incumbent paradigm: a linear chain of *Thought → Action (JSON tool call) → Observation → repeat*. The model's entire history of reasoning and tool results is appended to the prompt every turn. It is simple, inspectable, and works well for interactive, stateful tasks (editing files, running shell commands, browsing).

**RLM** (Zhang et al., arXiv:2512.24601) is a tree-structured inference paradigm. The model writes fenced `repl` blocks that manipulate an external REPL variable store and spawn recursive child calls. Because the full context lives in variables, the LM sees only constant-size metadata. This enables:

- **Native parallelism** via `rlm_query_batched`
- **10M+ token scale** (two orders of magnitude beyond the base window)
- **Cheap child models** for leaf work while a frontier model handles control

### Comparison Table

| Dimension | ReAct (today) | RLM (proposed) |
|---|---|---|
| **Structure** | Linear chain | Tree of recursive calls |
| **Parallelism** | Sequential (or heavy swarm tasks) | Native batched fan-out (up to 8 concurrent) |
| **State** | Monolithic prompt scratchpad | External REPL variables |
| **Tool interface** | JSON schema (`ToolUse`) | Fenced `repl` DSL blocks |
| **Child cost** | Full agent loop per child | Cheap `deepseek-v4-flash` subprocess calls |
| **Observability** | Linear transcript | JSONL tree trace (`--trace`) |
| **Best for** | Interactive, stateful, tool-driven work | Parallel analysis, long context, batch generation |

### The Hybrid Stance

We do not replace ReAct with RLM; we make RLM a first-class *primitive inside* the ReAct loop. The agent still reasons in natural language and calls tools via JSON when it needs interactive side effects. But when it wants to fan out parallel analysis, decompose a large context, or batch-generate, it writes a `repl` block instead of spawning `agent_swarm`. The engine detects the block, runs `zigrlm`, and feeds the aggregated `FINAL` result back as the assistant's answer for that turn.

This maps to the paper's own framing: RLM is the next milestone *after* CoT and ReAct, not a replacement for them.

---

## Appendix B: UI/UX Design

### The Core Tension

The Pro model streams its response naturally. The user sees "I'll break this into parallel searches…" and then watches a `\`\`\`repl` fence appear character-by-character. This is **good** — it shows intent and builds trust. But once the fence closes, the engine must pause, fork `zigrlm`, wait for N parallel flash calls to finish, and then present a single coherent answer. The UI must bridge that gap without feeling broken.

### The Solution: Progressive Disclosure via "Thinking Reclassification"

The existing TUI already has the perfect visual language for this: `HistoryCell::Thinking` (`crates/tui/src/tui/history.rs`, lines 1129–1198). Thinking blocks render with a left border (`▏`), a header showing a spinner and duration, collapsible by default, and markdown body. We reuse that pattern exactly.

**The flow:**

1. **Streaming phase** — The Pro model streams its response. The TUI shows it live in `HistoryCell::Assistant { streaming: true }`, exactly as today.
2. **Detection phase** — After `MessageStop`, the engine detects the repl block and emits `Event::RlmStarted { message_index }`.
3. **Reclassification** — The engine mutates `session.messages`:
   - The `ContentBlock::Text` containing the repl fence is moved to a new `ContentBlock::Thinking` block.
   - A transient `ContentBlock::Text` placeholder is inserted: "*Running RLM tree…*"
4. **Execution phase** — `zigrlm` runs. The TUI footer shows `RLM ⌀` (sky blue, same family as `working`). The transcript shows the thinking block collapsed with a live spinner: `◦ thinking live`.
5. **Completion phase** — `zigrlm` returns. The engine replaces the placeholder text with the `FINAL` result and emits `Event::RlmComplete { message_index, usage, duration_ms }`.
6. **Final render** — The TUI updates:
   - `ContentBlock::Text` now shows the aggregated result (normal markdown).
   - `ContentBlock::Thinking` shows the original repl plan, now collapsed and labeled `thinking done · 1.2s`.
   - A one-line metadata footer is appended: `▸ 3 flash calls · 2.1K tokens · ~$0.003`.

This gives the user **one thoughtful response**: the result is the message, and the repl block is the reasoning behind it — exactly how `Thinking` blocks work today.

### Visual States

| State | Transcript | Footer | Thinking Block |
|---|---|---|---|
| **Streaming** | Assistant cell, `streaming: true` | `thinking ⌀` | Not visible yet (model hasn't emitted fence) |
| **Executing** | Assistant cell, spinner suffix on placeholder | `recursing ⌀` | Collapsed, header reads `◦ recursing live` |
| **Complete** | Assistant cell, result text | Idle | Collapsed, header reads `◦ recursing done · 1.2s` |
| **Expanded** | Same | Idle | Expanded, shows full repl DSL with syntax highlighting |

### Why Not a Tool Card?

It is tempting to model RLM execution as a new `ToolCell` variant. We explicitly reject this because RLM is **not a tool call** — it is an inline primitive. Rendering it as a tool card would:
- Break the "single thoughtful response" metaphor
- Train the user to think of RLM as an external action rather than assistant reasoning
- Add visual noise (tool headers, argument summaries, result boxes) for what is essentially accelerated thinking

The `Thinking` block is the right container because the repl block *is* the model's reasoning about how to parallelize. The result is simply the output of that reasoning.

### Keyboard & Detail Views

- **`v` on the assistant message** — Opens the `PagerView` (`crates/tui/src/tui/pager.rs`) showing the full message: the original repl block at the top (with `zigrlm` trace path if available), the FINAL result below, and the JSONL tree if the user wants to inspect child calls.
- **`v` on the thinking block** — Toggles collapse/expand inline, same as existing thinking behavior.
- **`Alt+4` (sidebar)** — Future: an RLM panel showing recent RLM executions with call counts, depth, and trace file paths. Deferred to v0.6.

### Footer & Status Indicators

**New footer state** in `crates/tui/src/tui/ui.rs` (`footer_state_label()`, ~line 4022):

```rust
else if app.active_rlm.is_some() {
    ("recursing ⌀", Style::default().fg(Color::Sky))
}
```

The word *recursing* is playful, accurate, and short enough for the footer. Alternatives considered: `recursive thinking ⌀` (too long), `RLM ⌀` (too opaque). `recursing` wins because it describes what is actually happening — the model is recursively fanning out child calls — and it fits the existing informal voice of the TUI (`thinking ⌀`, `working`, `compacting ⌀`).

**Motion refresh:** The existing `UI_STATUS_ANIMATION_MS` (360 ms) timer already bumps the transcript cache when `history_has_live_motion` is true. We add `app.active_rlm.is_some()` to that check so the spinner animates while `zigrlm` runs.

### Events to Add

**File:** `crates/tui/src/core/events.rs`

```rust
pub enum Event {
    // ... existing variants
    RlmStarted {
        message_index: usize,
        estimated_calls: Option<usize>,
    },
    RlmComplete {
        message_index: usize,
        usage: Usage,
        duration_ms: u64,
    },
    RlmFailed {
        message_index: usize,
        error: String,
    },
}
```

### Future: Tree Visualization

`zigrlm` can emit `--trace /path/to/run.jsonl`. In v0.6 we can parse that JSONL and render a tree widget showing:

- Root prompt (depth 0)
- Each `rlm_query` / `rlm_query_batched` child (depth 1..N)
- Per-node usage (calls, tokens, cost)
- Duration bars

This would live in the `PagerView` or a dedicated sidebar panel, not in the main transcript. It is a debugging/observability feature, not part of the default conversation flow.

### Cost Accounting

`zigrlm` returns usage metadata per run (`calls`, `input_tokens`, `output_tokens`, `cost_micros`). The engine must fold this into the session's aggregate `Usage` in `crates/tui/src/core/session.rs`. However, there is a subtlety: the **root Pro call** that emitted the `repl` block is already counted as part of the normal assistant-message usage. `zigrlm` then performs its *own* root call (also Pro, because `ZIGRLM_MAIN_CMD` points at the session model) plus N child calls (flash). In practice this means the Pro prompt is billed twice — once by our client for the streaming turn, once by `zigrlm` for the root RLM call. This double-counting is acceptable for the spike, but Phase 2 should explore passing the *already-received* assistant text directly into `zigrlm` without re-billing the root call, or subtracting the overlap from displayed totals.

**Display policy:** Show raw numbers only (`3 flash calls · 2.1K tokens · ~$0.003`). Do **not** attempt to show "savings vs. ReAct" because that is a counterfactual — we cannot know how many Pro turns `agent_swarm` would have needed for the same task. The user can infer the value themselves: one Pro call + eight flash calls is visibly cheaper than five Pro calls.

### Anti-Patterns

- **Do not** stream `zigrlm` child progress into the transcript in real time. Flash calls complete in 1–3 seconds; the noise is not worth the signal.
- **Do not** show a modal or full-screen overlay during RLM execution. The user should be able to scroll, read history, and type the next query while `zigrlm` works.
- **Do not** render the raw `[0]\n…\n[1]\n…` batched response format directly. If `FINAL` was missing and we fall back to raw output, strip the indexed prefixes before displaying.
- **Do not** show fake "you saved $X.XX" badges. The comparison baseline is undefined and the math is misleading.

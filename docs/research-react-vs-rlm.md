# ReAct vs. Recursive Language Models (RLM): A Design Document Comparison

> **Purpose:** Provide the deepseek-tui team with a grounded, citation-rich comparison of the ReAct agent paradigm and the emerging Recursive Language Model (RLM) paradigm so that integration choices (e.g. `zigrlm`, `agent_swarm`, inline tool use) can be made deliberately.

---

## 1. ReAct: Reasoning + Acting

### 1.1 Origins and Definition

**ReAct** (Reason + Act) is a prompting and inference paradigm introduced by Yao et al. (Google Research / Princeton) and published at ICLR 2023. It unifies *reasoning traces* (chain-of-thought-style internal monologue) with *task-specific actions* (tool calls, API requests, environment commands) in a single autoregressive loop.  
**Citation:** Shunyu Yao et al., *"ReAct: Synergizing Reasoning and Acting in Language Models"*, ICLR 2023.

The core insight is that reasoning without acting suffers from fact hallucination and stale knowledge, while acting without reasoning lacks planning, error recovery, and interpretability. ReAct interleaves the two explicitly.

### 1.2 The Thought → Action → Observation Loop

At each timestep \(t\) the agent maintains a context \(c_t\) containing the original query and all prior tuples. The loop is:

1. **Thought** — The LLM generates a reasoning trace: plan decomposition, progress tracking, or exception handling.  
   \(c_t \rightarrow \text{Thought}_{t+1}\)
2. **Action** — Conditioned on the thought, the LLM emits a structured action (e.g. `Search[entity]`, `Calculator[expr]`, `Finish[answer]`).  
   \(c_{t+1} := c_t \parallel \text{Thought}_{t+1}\)
3. **Observation** — The action is executed in the external environment and the result is appended.  
   \(c_{t+1} := c_{t+1} \parallel \text{Action}_{t+1} \parallel \text{Obs}_{t+1}\)

The process halts when a special finish action is produced or a hard iteration limit is reached. Probabilistically this is:

\[
P(\tau \mid q) = \prod_{t=1}^{T} P(v_t \mid q, v_{<t})
\]

where \(v_t\) spans both Thought and Action tokens and \(\tau\) is the trajectory.

**Key traits:**
- **Linear / sequential** — Each observation must return before the next thought is generated.
- **Scratchpad-based** — The entire history of thoughts, actions, and observations is appended to the prompt; there is no external variable store.
- **Bounded by context window** — As the loop iterates, the prompt grows monotonically (until compaction heuristics truncate it).

### 1.3 Implementations in the Wild

| Framework | ReAct flavour |
|-----------|---------------|
| **OpenAI Function Calling** (and compatible APIs) | The model emits JSON `function_call` objects as Actions; tool results are fed back as `tool` role messages as Observations. The "Thought" is often implicit or rendered as a visible `<thinking>` block. |
| **LangChain / LangGraph** | Pre-built `ReAct` agent chain with a stop-and-observe parser. LangGraph generalises the loop into a state machine with nodes (Thought, Action, Observation) and conditional edges. |
| **LlamaIndex, BeeAI, etc.** | Provide pre-configured ReAct modules that wrap an LLM with a tool registry and a loop driver. |

A 2025–2026 refinement called **Focused ReAct** presets the original query at each step to prevent drift, reportedly improving accuracy by >5× and reducing runtime by ~34%.

---

## 2. Recursive Language Models (RLM)

### 2.1 Origins and Definition

**RLM** is a general *inference-time scaling* paradigm proposed by Zhang, Kraska, and Khattab (MIT CSAIL) in late 2025. Rather than viewing the user prompt as static input tokens, RLMs treat the prompt as part of an **external environment** that the model can programmatically examine, decompose, and recursively query.  
**Citation:** Alex L. Zhang, Tim Kraska, and Omar Khattab, *"Recursive Language Models"*, arXiv:2512.24601 [cs.AI], 2025 (v2 Jan 2026).

A second formalisation, **λ-RLM**, refines the open-ended code generation of the original paper into a deterministic λ-calculus combinator runtime (SPLIT, PEEK, MAP, FILTER, REDUCE, CONCAT) to eliminate brittle free-form generation.  
**Citation:** *"Solving Long-Context Rot with λ-Calculus"*, arXiv:2603.20105, 2026.

### 2.2 The REPL Environment and Recursive Call Model

The canonical RLM implementation wraps the root LM in a read-eval-print loop (REPL) — usually Python, though Clojure (`loop-infer`) and bash (`claude-rlm`) adaptations exist. The full context is stored as a variable (e.g. `context`) in the REPL, **not** in the model's prompt window.

At each root iteration:
1. The LM receives only *metadata* about the REPL state (short stdout prefix, variable names).
2. The LM emits **code** (or fenced `repl` directives) that manipulate the variable, run regex/grep, or spawn recursive sub-calls.
3. The code executes; stdout and updated variables are captured.
4. The loop repeats until the LM sets a special `Final` variable (or emits `FINAL(...)` / `FINAL_VAR(...)`), at which point the run returns.

Because the full text never enters the root LM context window, RLMs can scale to **10M+ tokens** (two orders of magnitude beyond the base model's window) without retraining.

### 2.3 The `repl` Grammar and Tree Structure

In the `zigrlm` runtime (and the reference Python implementation), the root LM writes fenced blocks tagged `repl`. The grammar includes:

| Directive | Semantics |
|-----------|-----------|
| `let name = "..."` | Bind a string variable |
| `js name = "...FINAL(...)"` | Execute deterministic JS in a sandbox |
| `llm_query name = expr` | Call the *same* model (same depth) |
| `rlm_query name = expr` | Spawn a **child** RLM (depth + 1) |
| `llm_query_batched name = a \| b \| c` | Parallel same-depth calls |
| `rlm_query_batched name = a \| b \| c` | Parallel child RLMs |
| `FINAL(expression)` | Terminate and return this string |
| `FINAL_VAR(name)` | Terminate and return the named variable |

These recursive calls form a **tree of reasoning**, not a single chain. Each child processes a snippet of the external context and stores its partial result back into a parent REPL variable. Aggregation is performed programmatically (lists, tallies, tables) rather than autoregressively.

**Key traits:**
- **Context-centric decomposition** — The model decides how to slice the *input context*, not just how to sequence actions.
- **Variable store** — Intermediate results live in the REPL, keeping the LM context window constant-size.
- **Bounded output** — Because `Final` can be assembled from variables, RLMs can produce answers longer than the model's output token limit.

### 2.4 Implementations and Ecosystem

| Project | Notes |
|---------|-------|
| **alexzhang13/rlm** | Official research repo (Python). Includes reference REPL, natively fine-tuned `RLM-Qwen3-8B`, and OOLONG / BrowseComp-Plus benchmarks. |
| **alexzhang13/rlm-minimal** | Stripped-down Python version for hacking. |
| **zigrlm** | Zig-native runtime with JS sandbox, batched parallel fan-out, and JSONL tracing. Used by deepseek-tui for cheap `deepseek-v4-flash` child dispatch. |
| **claude-rlm** | Depth-N recursion using Claude Code instances as sub-agents; bash-as-REPL; `mkdir`-based concurrency limiter. |
| **loop-infer** | Clojure REPL implementation. |
| **minrlm** | Independent minimal RLM reducing token usage up to 4× vs. flat inference. |
| **rlm-mcp** | MCP server wrapper exposing RLM through the Model Context Protocol. |

---

## 3. Key Differences

### 3.1 Parallelism

| Dimension | ReAct | RLM |
|-----------|-------|-----|
| **Structure** | Linear chain. Each Action depends on the prior Observation. | Tree. A parent can fan out N children in parallel. |
| **Batched execution** | Not native. Some frameworks (LangGraph) add parallel branches, but the canonical ReAct loop is sequential. | Native via `*_batched` directives. `zigrlm` dispatches children across OS threads capped by `max_concurrent_subcalls`. |
| **Synchronisation** | Implicit: the loop blocks on the environment. | Children write to named variables; parent continues only after aggregation code runs. |

The RLM paper explicitly notes that their reference implementation used *blocking* sequential sub-calls and left async fan-out as "low-hanging fruit" for systems builders. `zigrlm` realises that fruit.

### 3.2 Reasoning Representation

| Dimension | ReAct | RLM |
|-----------|-------|-----|
| **Form** | Natural-language "Thought" traces appended to a scratchpad. | Code / DSL inside fenced `repl` blocks, plus natural-language plan text outside the fence. |
| **State management** | Monolithic prompt history. Intermediate values are re-tokenised every turn. | External REPL variables. The LM sees only constant-size metadata. |
| **Aggregation** | The model must autoregressively synthesise the final answer from the scratchpad. | Programmatic: `FINAL_VAR(tally)` or `FINAL("\n".join(results))`. |
| **Length limits** | Bounded by context window for both input and output. | Input: theoretically unbounded (10M+ tested). Output: bounded only by REPL variable memory. |

### 3.3 Tool Use

| Dimension | ReAct | RLM |
|-----------|-------|-----|
| **Interface** | Structured JSON schemas (OpenAI function calling) or text parsing (LangChain). | Natural-language fenced blocks (`repl`). The "tool" is the REPL itself. |
| **Tool set** | Fixed registry of functions known at build time. | Open-ended: the LM can write arbitrary regex, loops, or JS to manipulate data. |
| **Child agents** | Spawning a sub-agent is a heavyweight Action (new thread/process, full tool registry, event channels). | Spawning a child is a lightweight `rlm_query` inside the same runtime; the child uses a cheaper model by default. |

### 3.4 Cost Model

| Dimension | ReAct | RLM |
|-----------|-------|-----|
| **Primary model** | Usually one expensive frontier model (e.g. GPT-5, Claude Opus, deepseek-v4-pro) for every turn. | A **root** model (frontier) for control + cheap **child** models (`deepseek-v4-flash`, GPT-5-mini) for sub-tasks. |
| **Cost scaling** | Grows with iteration count × full prompt length. Compaction heuristics trade quality for cost. | Grows with *task complexity*, not input length. Selective inspection means most tokens are never fed to the LM. |
| **Empirical results** | N/A (baseline). | On OOLONG 128K, `RLM(GPT-5-mini)` outperformed flat `GPT-5` by >2× and was cheaper on average. On BrowseComp-Plus (1K docs, 6–11M tokens), RLM(GPT-5) averaged **$0.99** vs. $1.50–$2.75 for the base model ingesting everything. |
| **Variance** | Predictable per-turn cost. | High variance: some trajectories are cheaper than a flat call, outliers can be more expensive. |

### 3.5 Observability

| Dimension | ReAct | RLM |
|-----------|-------|-----|
| **Trace shape** | Linear log of (Thought, Action, Observation) tuples. | Tree log: each node is a REPL turn that may branch into child RLM nodes. |
| **Depth** | Flat iteration count. | Explicit recursion depth (`max_depth`). |
| **Tooling** | LangSmith, OpenTelemetry spans, simple print logging. | JSONL trace files (`--trace`) capturing every code cell, stdout snapshot, and sub-call with usage metadata. |
| **Human readability** | Easy: read the scratchpad top-to-bottom. | Harder: requires tree traversal, but the `FINAL` node summarises the aggregate. |

---

## 4. When Is Each Appropriate? Trade-offs

### Use ReAct when …
- The task is **interactive and stateful** (e.g. browsing, CLI commands, file editing) where each observation is dynamic and the next action cannot be predicted ahead of time.
- The tool surface is **fixed and schema-driven** (e.g. a known set of REST APIs, file-system operations, database queries).
- You need **deterministic latency bounds** per turn (e.g. a chat UI that must stream a Thought before the next Action).
- The context fits comfortably within the model's window and does not suffer from context rot.
- Human inspectability of a single linear reasoning chain is a priority.

### Use RLM when …
- The input is **very long** (100K–10M+ tokens) and you want to avoid summarisation or compaction loss.
- The work is **embarrassingly parallel** (e.g. classify 1,000 rows, evaluate 50 files, score 20 answers). `rlm_query_batched` maps naturally.
- The task is **recursively decomposable** (e.g. divide-and-conquer summarisation, map-reduce aggregation, multi-hop retrieval over a corpus).
- Cost is a constraint: you can offload leaf work to a **cheap child model** while reserving the frontier model for control decisions.
- You need **deterministic local compute** interleaved with model calls (JS / Python in the REPL).

### Hybrids
There is no forced binary choice. A pragmatic system (like deepseek-tui) can use:
- **ReAct / OpenAI-style function calling** for interactive tool use and user-facing chat turns.
- **RLM `repl` blocks** for internal parallel decomposition, batched generation, or long-context analysis.
- **Agent swarm** (multi-step ReAct sub-agents) only when autonomous, stateful, multi-tool workflows are required.

The RLM paper itself positions RLMs as the next milestone *after* CoT-style reasoning and ReAct-style agents, not as a replacement for them.

---

## 5. Bibliography

1. **Yao, S. et al.** *ReAct: Synergizing Reasoning and Acting in Language Models.* ICLR 2023.  
   - Blog explainer: https://www.promptingguide.ai/techniques/react  
   - IBM overview: https://www.ibm.com/think/topics/react-agent

2. **Zhang, A. L., Kraska, T., and Khattab, O.** *Recursive Language Models.* arXiv:2512.24601 [cs.AI], 2025 (v2 Jan 2026).  
   - Paper: https://arxiv.org/abs/2512.24601  
   - Blog: https://alexzhang13.github.io/blog/2025/rlm/  
   - Code: https://github.com/alexzhang13/rlm  
   - Minimal code: https://github.com/alexzhang13/rlm-minimal

3. **λ-RLM authors.** *Solving Long-Context Rot with λ-Calculus.* arXiv:2603.20105, 2026.  
   - Formalises RLM control into typed combinators (SPLIT, MAP, FILTER, REDUCE) to replace free-form code generation.

4. **zigrlm** (Zig RLM runtime). Local build: `/Volumes/VIXinSSD/zigrlm/zig-out/bin/zigrlm`.  
   - Supports `cli`, `cli-claude`, `cli-codex`, `cli-openai`, `zai`, `openai-proxy`, etc.  
   - Grammar: fenced `repl` blocks with `rlm_query`, `rlm_query_batched`, `FINAL`, `FINAL_VAR`.

5. **Community implementations and extensions**  
   - `claude-rlm` (depth-N recursion via Claude Code + bash): https://github.com/Tenobrus/claude-rlm  
   - `minrlm` (token-reduction focus): https://github.com/avilum/minrlm  
   - `loop-infer` (Clojure REPL): https://github.com/unravel-team/loop-infer  
   - `rlm-mcp` (MCP server): https://github.com/eesb99/rlm-mcp  
   - `rlm_repl` (Python PoC): https://github.com/fullstackwebdev/rlm_repl

6. **Benchmarks referenced**  
   - **OOLONG** (long-context aggregation): Bertsch et al., 2025.  
   - **BrowseComp-Plus** (multi-hop QA over document corpora): Chen et al., 2025.

---

*Document generated for deepseek-tui design review. Corresponds to repo state: main @ 229b1993.*

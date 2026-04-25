# DeepSeek TUI

> **A terminal-native coding agent for [DeepSeek V4](https://platform.deepseek.com) models — with 1M-token context, thinking-mode reasoning, and full tool-use.**

```bash
npm i -g deepseek-tui
```

[![CI](https://github.com/Hmbown/DeepSeek-TUI/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/DeepSeek-TUI/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/deepseek-tui)](https://crates.io/crates/deepseek-tui)
[![npm](https://img.shields.io/npm/v/deepseek-tui)](https://www.npmjs.com/package/deepseek-tui)

---

## What is it?

DeepSeek TUI is a coding agent that runs entirely in your terminal. It gives DeepSeek's frontier models direct access to your workspace — reading and editing files, running shell commands, searching the web, managing git, and orchestrating sub-agents — all through a fast, keyboard-driven TUI.

**Built for DeepSeek V4** (`deepseek-v4-pro` / `deepseek-v4-flash`) with 1M-token context windows and native thinking-mode (chain-of-thought) streaming. See the model's reasoning unfold in real time as it works through your tasks.

### Key Features

- 🧠 **Thinking-mode streaming** — watch DeepSeek's chain-of-thought as it reasons about your code
- 🔧 **Full tool suite** — file ops, shell execution, git, web search/browse, apply-patch, sub-agents, MCP servers, and more
- 🪟 **1M-token context** — feed entire codebases; automatic intelligent compaction when context fills up
- 🎛️ **Three interaction modes** — Plan (read-only explore), Agent (interactive with approval), YOLO (auto-approved)
- ⚡ **Reasoning-effort tiers** — cycle through `off → high → max` with Shift+Tab
- 🔄 **Session save/resume** — checkpoint and resume long sessions, fork conversations
- 🌐 **HTTP/SSE runtime API** — `deepseek serve --http` for headless agent workflows
- 📦 **MCP protocol** — connect to Model Context Protocol servers for extended tooling
- 💰 **Live cost tracking** — per-turn and session-level token usage and cost estimates
- 🎨 **Dark & light themes** — with a DeepSeek-blue branded palette

---

## Quickstart

```bash
npm install -g deepseek-tui
deepseek
```

On first launch you'll be prompted for your [DeepSeek API key](https://platform.deepseek.com/api_keys). You can also set it ahead of time:

```bash
# via CLI
deepseek login --api-key "YOUR_DEEPSEEK_API_KEY"

# via env var
export DEEPSEEK_API_KEY="YOUR_DEEPSEEK_API_KEY"
deepseek
```

### Using NVIDIA NIM

```bash
deepseek auth set --provider nvidia-nim --api-key "YOUR_NVIDIA_API_KEY"
deepseek --provider nvidia-nim

# or per-process:
DEEPSEEK_PROVIDER=nvidia-nim NVIDIA_API_KEY="..." deepseek
```

<details>
<summary>Other install methods</summary>

```bash
# From crates.io (requires Rust 1.85+)
cargo install deepseek-tui --locked       # TUI binary
cargo install deepseek-tui-cli --locked   # CLI facade (deepseek command)

# From source
git clone https://github.com/Hmbown/DeepSeek-TUI.git
cd DeepSeek-TUI
cargo install --path crates/tui --locked
```

The canonical crates.io packages are `deepseek-tui` and `deepseek-tui-cli`. The unrelated `deepseek-cli` crate is not part of this project. crates.io publication can lag the workspace version — use npm or install from source for the latest release surface immediately.

</details>

---

## Models & Pricing

DeepSeek TUI targets **DeepSeek V4** models with 1M-token context windows by default.

| Model | Context | Input (cache hit) | Input (cache miss) | Output |
|---|---|---|---|---|
| `deepseek-v4-pro` | 1M | $0.03625 / 1M* | $0.435 / 1M* | $0.87 / 1M* |
| `deepseek-v4-flash` | 1M | $0.028 / 1M | $0.14 / 1M | $0.28 / 1M |

Legacy aliases `deepseek-chat` and `deepseek-reasoner` silently map to `deepseek-v4-flash`.

**NVIDIA NIM** hosted variants (`deepseek-ai/deepseek-v4-pro`, `deepseek-ai/deepseek-v4-flash`) use your NVIDIA account terms — no DeepSeek platform billing.

*\*DeepSeek lists the Pro rates above as a limited-time 75% discount valid until 2026-05-05 15:59 UTC; the TUI cost estimator falls back to base Pro rates after that timestamp.*

---

## Usage

```bash
deepseek                                      # interactive TUI
deepseek "explain this function"              # one-shot prompt
deepseek --model deepseek-v4-flash "summarize" # model override
deepseek --yolo                               # YOLO mode (auto-approve tools)
deepseek login --api-key "..."                # save API key
deepseek doctor                               # check setup & connectivity
deepseek models                               # list live API models
deepseek sessions                             # list saved sessions
deepseek resume --last                        # resume latest session
deepseek serve --http                         # HTTP/SSE API server
```

### Keyboard shortcuts

| Key | Action |
|---|---|
| `Tab` | Cycle mode: Plan → Agent → YOLO |
| `Shift+Tab` | Cycle reasoning-effort: off → high → max |
| `F1` | Help |
| `Esc` | Back / dismiss |
| `Ctrl+K` | Command palette |
| `@path` | Attach file/directory context in composer |
| `/attach <path>` | Attach image/video media references |

---

## Modes

| Mode | Behavior |
|---|---|
| **Plan** 🔍 | Read-only investigation — model explores and proposes a plan before making changes |
| **Agent** 🤖 | Default interactive mode — multi-step tool use with approval gates |
| **YOLO** ⚡ | Auto-approve all tools in a trusted workspace (use with caution) |

---

## Configuration

`~/.deepseek/config.toml` — see [config.example.toml](config.example.toml) for every option.

Key environment overrides:

| Variable | Purpose |
|---|---|
| `DEEPSEEK_API_KEY` | API key |
| `DEEPSEEK_BASE_URL` | API base URL |
| `DEEPSEEK_MODEL` | Default model |
| `DEEPSEEK_PROVIDER` | Provider: `deepseek` (default) or `nvidia-nim` |
| `DEEPSEEK_PROFILE` | Config profile name |
| `NVIDIA_API_KEY` | NVIDIA NIM API key |

Quick diagnostics:

```bash
deepseek-tui setup --status    # read-only status check (API key, MCP, sandbox, .env)
deepseek-tui doctor --json     # machine-readable doctor output for CI
deepseek-tui setup --tools --plugins  # scaffold tools/ and plugins/ directories
```

DeepSeek context caching is automatic — when the API returns cache hit/miss token fields, the TUI includes them in usage and cost tracking.

Full reference: [docs/CONFIGURATION.md](docs/CONFIGURATION.md)

---

## Documentation

| Doc | Topic |
|---|---|
| [ARCHITECTURE.md](docs/ARCHITECTURE.md) | Codebase internals |
| [CONFIGURATION.md](docs/CONFIGURATION.md) | Full config reference |
| [MODES.md](docs/MODES.md) | Plan / Agent / YOLO modes |
| [MCP.md](docs/MCP.md) | Model Context Protocol integration |
| [RUNTIME_API.md](docs/RUNTIME_API.md) | HTTP/SSE API server |
| [RELEASE_RUNBOOK.md](docs/RELEASE_RUNBOOK.md) | Release process |
| [OPERATIONS_RUNBOOK.md](docs/OPERATIONS_RUNBOOK.md) | Ops & recovery |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Pull requests welcome!

*Not affiliated with DeepSeek Inc.*

## License

[MIT](LICENSE)

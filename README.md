# Codewhale

A coding agent for your terminal. Point it at a model — DeepSeek, Claude,
GPT, Kimi, GLM, 30+ hosted providers, or your own vLLM/SGLang/Ollama, no
key required — and give it a task. It reads your code, edits files, runs
commands, checks its work, and stops when it's done or needs you. Switch
models mid-task with `/model`. Use the TUI for interactive work,
`codewhale exec` for scripts and CI.

Plan mode is read-only. Approvals gate risky commands, and a repo's
`constitution.json` can pin write holds that even Full Access can't skip.
Fleets log every step to a ledger, so `fleet resume` picks up where you
left off.

Rust, MIT, runs on your machine. Started as `deepseek-tui`; renamed once
the community wanted more providers than one.

[简体中文](README.zh-CN.md) · [日本語](README.ja-JP.md) · [Tiếng Việt](README.vi.md) · [한국어](README.ko-KR.md) · [Español](README.es-419.md) · [Português](README.pt-BR.md) · [codewhale.net](https://codewhale.net/) · [Docs](docs) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)

![Codewhale running in a terminal](assets/screenshot.png)

## Install

```bash
npm install -g codewhale
```

Cargo, Docker, Nix, Scoop, prebuilt archives, Android/Termux, and a CNB mirror
for users who cannot reach GitHub are covered in
[docs/INSTALL.md](docs/INSTALL.md). Coming from `deepseek-tui`? Your config and
sessions carry over — see [docs/REBRAND.md](docs/REBRAND.md).

## Use

```bash
codewhale auth set --provider deepseek   # or export ANTHROPIC_API_KEY, etc.
codewhale                                # open the TUI
codewhale exec "fix the failing test"    # headless
codewhale web                            # local browser client on 127.0.0.1
```

In the TUI: `/model` switches provider and model together, `/fleet` runs a
team of workers, and `/restore` undoes a turn. When the composer is idle, `Tab`
cycles Plan / Act / Operate and `Shift+Tab` cycles the Ask / Auto-Review / Full
Access permission posture. `!` runs a shell command through the normal approval
path.

## Learn more

- [docs/PROVIDERS.md](docs/PROVIDERS.md) — every provider route: hosted,
  gateway, and local
- [docs/FLEET.md](docs/FLEET.md) — fleets, the ledger, and resume
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — `config.toml`, hooks, and
  the constitution
- [docs/WEB.md](docs/WEB.md) — the loopback-only embedded browser client and
  its one-time authentication boundary

Everything else — modes, keybindings, sandbox details, MCP, the runtime API,
architecture — is in [docs](docs) and on
[codewhale.net](https://codewhale.net/).

## Contributing

Issues, PRs, repro steps, and feature requests are all welcome. When a PR
can't merge as-is, maintainers harvest what works and credit the author in
the commit, the changelog, and [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md).
Missing a provider, or something broke on your machine? Tell us.

- [Open issues](https://github.com/Hmbown/CodeWhale/issues) — good first
  contributions live here
- [CONTRIBUTING.md](CONTRIBUTING.md) — dev setup and PR flow
- [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) — everyone who has shaped this
- [Buy me a coffee](https://www.buymeacoffee.com/hmbown)

Thanks to [DeepSeek](https://github.com/deepseek-ai) for the models and support
that started the project, [DataWhale](https://github.com/datawhalechina) 🐋 for
welcoming us into the Whale Brother family, and
[OpenWarp](https://github.com/zerx-lab/warp) and
[Open Design](https://github.com/nexu-io/open-design) for collaborating on the
terminal-agent experience.

## License

[MIT](LICENSE). Independent community project; not affiliated with any model
provider.

[![Star History Chart](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date)

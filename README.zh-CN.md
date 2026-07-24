<!-- source: README.md sha256:ff4c58eb428c -->
# Codewhale

Codewhale 是运行在终端里的编程智能体。给它指定一个模型——DeepSeek、Claude、GPT、Kimi、GLM 等 30 多家托管 provider，或者你自己的 vLLM/SGLang/Ollama，无需 key——再交给它一个任务。它会读你的代码、改文件、跑命令、检查自己的工作，在任务完成或需要你介入时停下。任务中途用 `/model` 切换模型。交互式工作用 TUI，脚本和 CI 用 `codewhale exec`。

Plan 模式只读。审批把关高风险命令，仓库的 `constitution.json` 可以设定写入拦截，连 Full Access 也无法跳过。Fleet 把每一步记录到账本里，`fleet resume` 从你停下的地方继续。

Rust 编写，MIT 许可，运行在你自己的机器上。前身是 `deepseek-tui`；社区需要不止一家 provider，于是改了名字。

[English](README.md) · [日本語](README.ja-JP.md) · [Tiếng Việt](README.vi.md) · [한국어](README.ko-KR.md) · [Español](README.es-419.md) · [Português](README.pt-BR.md) · [codewhale.net](https://codewhale.net/) · [Docs](docs) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)

![Codewhale 在终端中运行](assets/screenshot.png)

## 安装

```bash
npm install -g codewhale
```

Cargo、Docker、Nix、Scoop、预编译归档、Android/Termux，以及面向无法访问 GitHub 用户的 CNB 镜像，均见 [docs/INSTALL.md](docs/INSTALL.md)。从 `deepseek-tui` 迁移过来？你的配置和会话可以直接沿用——见 [docs/REBRAND.md](docs/REBRAND.md)。

## 使用

```bash
codewhale auth set --provider deepseek   # or export ANTHROPIC_API_KEY, etc.
codewhale                                # open the TUI
codewhale exec "fix the failing test"    # headless
codewhale web                            # local browser client on 127.0.0.1
```

在 TUI 中：`/model` 同时切换 provider 和模型，`/fleet` 运行一组 worker，`/restore` 撤销某一轮。输入区空闲时，`Tab` 在 Plan / Act / Operate 之间循环切换，`Shift+Tab` 在 Ask / Auto-Review / Full Access 权限姿态之间循环切换。`!` 让 shell 命令经由正常的审批路径运行。

## 了解更多

- [docs/PROVIDERS.md](docs/PROVIDERS.md) — 每一条 provider 路由：托管、网关与本地
- [docs/FLEET.md](docs/FLEET.md) — Fleet、账本与恢复
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — `config.toml`、hooks 与 constitution
- [docs/WEB.md](docs/WEB.md) — 仅限回环地址的内置浏览器客户端及其一次性身份验证边界

其余内容——模式、键位绑定、沙箱细节、MCP、运行时 API、架构——见 [docs](docs) 与 [codewhale.net](https://codewhale.net/)。

## 贡献

欢迎提交 Issue、PR、复现步骤和功能请求。当一个 PR 无法原样合并时，维护者会吸收其中可用的部分，作者的署名会保留——在提交、更新日志和 [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) 中。缺少你在用的 provider，或者有什么东西在你机器上坏了？告诉我们。

- [开放 issue](https://github.com/Hmbown/CodeWhale/issues) —— 适合入门的贡献在这里
- [CONTRIBUTING.md](CONTRIBUTING.md) —— 开发环境搭建与 PR 流程
- [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) —— 每一位塑造过这个项目的人
- [Buy me a coffee](https://www.buymeacoffee.com/hmbown)

感谢 [DeepSeek](https://github.com/deepseek-ai) 提供让项目起步的模型与支持，感谢 [DataWhale](https://github.com/datawhalechina) 🐋 欢迎我们加入“鲸兄弟”大家庭，也感谢 [OpenWarp](https://github.com/zerx-lab/warp) 与 [Open Design](https://github.com/nexu-io/open-design) 在终端智能体体验上的协作。

## 许可证

[MIT](LICENSE)。独立的社区项目，与任何模型 provider 均无隶属关系。

[![Star History Chart](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date)

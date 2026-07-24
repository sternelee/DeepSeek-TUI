<!-- source: README.md sha256:ff4c58eb428c -->
# Codewhale

Codewhale はターミナルで動くコーディングエージェントです。DeepSeek、Claude、
GPT、Kimi、GLM など 30 以上のホスト型プロバイダ、あるいは自前の
vLLM/SGLang/Ollama(キー不要)——モデルを指定してタスクを渡せば、コードを読み、
ファイルを編集し、コマンドを実行し、自分の作業を確認して、完了するかあなたの
判断が必要になった時点で止まります。タスクの途中でも `/model` でモデルを
切り替えられます。対話的な作業には TUI を、スクリプトと CI には
`codewhale exec` を使います。

Plan モードは読み取り専用です。承認がリスクのあるコマンドをゲートし、
リポジトリの `constitution.json` で Full Access でも突破できない書き込み
制限を設定できます。Fleet は各ステップを台帳に記録するので、`fleet resume`
で中断した場所から再開できます。

Rust 製、MIT ライセンス、あなたのマシン上で動きます。前身は `deepseek-tui`。
コミュニティがより多くのプロバイダを求めたため、改名しました。

[English](README.md) · [简体中文](README.zh-CN.md) · [Tiếng Việt](README.vi.md) · [한국어](README.ko-KR.md) · [Español](README.es-419.md) · [Português](README.pt-BR.md) · [codewhale.net](https://codewhale.net/) · [Docs](docs) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)

![ターミナルで動作する Codewhale](assets/screenshot.png)

## インストール

```bash
npm install -g codewhale
```

Cargo、Docker、Nix、Scoop、ビルド済みアーカイブ、Android/Termux、そして GitHub に到達できないユーザー向けの CNB ミラーについては [docs/INSTALL.md](docs/INSTALL.md) で扱っています。`deepseek-tui` からの移行なら、設定とセッションはそのまま引き継がれます — [docs/REBRAND.md](docs/REBRAND.md) を参照してください。

## 使い方

```bash
codewhale auth set --provider deepseek   # or export ANTHROPIC_API_KEY, etc.
codewhale                                # open the TUI
codewhale exec "fix the failing test"    # headless
codewhale web                            # local browser client on 127.0.0.1
```

TUI では、`/model` がプロバイダとモデルをまとめて切り替え、`/fleet` がワーカーのチームを走らせ、`/restore` がターンを取り消します。入力欄がアイドル状態のとき、`Tab` は Plan / Act / Operate を順に切り替え、`Shift+Tab` は Ask / Auto-Review / Full Access の権限スタンスを順に切り替えます。`!` は Shell コマンドを通常の承認経路で実行します。

## さらに詳しく

- [docs/PROVIDERS.md](docs/PROVIDERS.md) — ホスト型・ゲートウェイ・ローカル
  まで、すべてのプロバイダルート
- [docs/FLEET.md](docs/FLEET.md) — Fleet、台帳、再開
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — `config.toml`、フック、
  constitution
- [docs/WEB.md](docs/WEB.md) — ループバック専用の組み込みブラウザクライアントと
  ワンタイム認証境界

その他 — モード、キーバインド、サンドボックスの詳細、MCP、ランタイム API、
アーキテクチャ — は [docs](docs) と [codewhale.net](https://codewhale.net/)
にあります。

## コントリビューション

Issue、PR、再現手順、機能要望はいつでも歓迎です。PR がそのままマージできない場合、メンテナは使える部分を収穫（harvest）し、作者のクレジットは残ります — コミットにも、changelog にも、[docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) にも。使っているプロバイダが見当たらない、あるいは手元のマシンで何かが壊れた?それを教えてください。

- [Open issues](https://github.com/Hmbown/CodeWhale/issues) — 最初のコントリビューションに向くものはここにあります
- [CONTRIBUTING.md](CONTRIBUTING.md) — 開発環境のセットアップと PR の流れ
- [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) — このプロジェクトを形づくってきた全員
- [Buy me a coffee](https://www.buymeacoffee.com/hmbown)

プロジェクトの出発点となったモデルとサポートを提供してくれた [DeepSeek](https://github.com/deepseek-ai)、「鯨兄弟」ファミリーに迎え入れてくれた [DataWhale](https://github.com/datawhalechina) 🐋、そしてターミナルエージェント体験で協力してくれている [OpenWarp](https://github.com/zerx-lab/warp) と [Open Design](https://github.com/nexu-io/open-design) に感謝します。

## ライセンス

[MIT](LICENSE)。独立したコミュニティプロジェクトであり、いかなるモデルプロバイダとも提携していません。

[![Star History Chart](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date)

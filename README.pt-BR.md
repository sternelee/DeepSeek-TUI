<!-- source: README.md sha256:ff4c58eb428c -->
# Codewhale

O Codewhale é um agente de código para o seu terminal. Aponte-o para um
modelo — DeepSeek, Claude, GPT, Kimi, GLM, mais de 30 provedores hospedados,
ou seu próprio vLLM/SGLang/Ollama, sem key — e dê uma tarefa a ele. Ele lê
seu código, edita arquivos, executa comandos, verifica o próprio trabalho e
para quando a tarefa termina ou quando precisa de você. Troque de modelo no
meio da tarefa com `/model`. Use a TUI para trabalho interativo e
`codewhale exec` para scripts e CI.

O modo Plan é somente leitura. Aprovações controlam comandos arriscados, e o
`constitution.json` de um repositório pode fixar bloqueios de escrita que
nem o Full Access consegue pular. Fleets registram cada passo em um
livro-razão, então `fleet resume` retoma de onde você parou.

Rust, MIT, roda na sua máquina. Nasceu como `deepseek-tui`; mudou de nome
quando a comunidade precisou de mais de um provedor.

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja-JP.md) · [Tiếng Việt](README.vi.md) · [한국어](README.ko-KR.md) · [Español](README.es-419.md) · [codewhale.net](https://codewhale.net/) · [Docs](docs) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)

![Codewhale rodando em um terminal](assets/screenshot.png)

## Instalação

```bash
npm install -g codewhale
```

Cargo, Docker, Nix, Scoop, arquivos pré-compilados, Android/Termux e um
espelho CNB para quem não consegue acessar o GitHub estão cobertos em
[docs/INSTALL.md](docs/INSTALL.md). Vindo do `deepseek-tui`? Sua configuração
e suas sessões são preservadas — veja [docs/REBRAND.md](docs/REBRAND.md).

## Uso

```bash
codewhale auth set --provider deepseek   # or export ANTHROPIC_API_KEY, etc.
codewhale                                # open the TUI
codewhale exec "fix the failing test"    # headless
codewhale web                            # local browser client on 127.0.0.1
```

Na TUI: `/model` troca provedor e modelo juntos, `/fleet` executa uma equipe
de workers e `/restore` desfaz um turno. Quando o compositor está ocioso, `Tab`
cicla entre Plan / Act / Operate e `Shift+Tab` cicla a postura de permissão Ask
/ Auto-Review / Full Access. `!` executa um comando de shell pelo caminho normal
de aprovação.

## Saiba mais

- [docs/PROVIDERS.md](docs/PROVIDERS.md) — cada rota de provedor: hospedada,
  gateway e local
- [docs/FLEET.md](docs/FLEET.md) — fleets, o livro-razão e resume
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — `config.toml`, hooks e a
  constitution
- [docs/WEB.md](docs/WEB.md) — cliente de navegador incorporado apenas em
  loopback e sua fronteira de autenticação de uso único

Todo o resto — modos, atalhos de teclado, detalhes do sandbox, MCP, a API do
runtime, arquitetura — está em [docs](docs) e em
[codewhale.net](https://codewhale.net/).

## Contribuindo

Issues, PRs, passos de reprodução e pedidos de funcionalidade são bem-vindos.
Quando um PR não pode ser mesclado como está, os mantenedores aproveitam o
que funciona e o autor continua creditado — no commit, no changelog e em
[docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md). Falta um provedor que você usa,
ou algo quebrou na sua máquina? Nos conte.

- [Issues abertas](https://github.com/Hmbown/CodeWhale/issues) — boas
  primeiras contribuições moram aqui
- [CONTRIBUTING.md](CONTRIBUTING.md) — setup de desenvolvimento e fluxo de PR
- [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) — todo mundo que ajudou a
  moldar o projeto
- [Me pague um café](https://www.buymeacoffee.com/hmbown)

Obrigado à [DeepSeek](https://github.com/deepseek-ai) pelos modelos e pelo
apoio que deram início ao projeto, à
[DataWhale](https://github.com/datawhalechina) 🐋 por nos receber na família
Whale Brother, e a [OpenWarp](https://github.com/zerx-lab/warp) e
[Open Design](https://github.com/nexu-io/open-design) pela colaboração na
experiência de agente no terminal.

## Licença

[MIT](LICENSE). Projeto comunitário independente; sem afiliação com nenhum
provedor de modelos.

[![Gráfico de Star History](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date)

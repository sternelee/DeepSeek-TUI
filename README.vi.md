<!-- source: README.md sha256:ff4c58eb428c -->
# Codewhale

Codewhale là một coding agent cho terminal của bạn. Chỉ định một model —
DeepSeek, Claude, GPT, Kimi, GLM, hơn 30 provider hosted, hoặc vLLM/SGLang/
Ollama của riêng bạn, không cần key — rồi giao cho nó một nhiệm vụ. Nó đọc
code của bạn, sửa file, chạy lệnh, kiểm tra công việc của mình, và dừng lại
khi nhiệm vụ hoàn thành hoặc cần đến bạn. Đổi model giữa chừng bằng `/model`.
Dùng TUI cho công việc tương tác, `codewhale exec` cho script và CI.

Chế độ Plan chỉ đọc. Phê duyệt gác mọi lệnh rủi ro, và `constitution.json`
của repo có thể chốt các giới hạn ghi mà ngay cả Full Access cũng không thể
bỏ qua. Fleet ghi lại từng bước vào sổ cái, nên `fleet resume` tiếp tục từ
chỗ bạn dừng.

Viết bằng Rust, giấy phép MIT, chạy trên máy của bạn. Sinh ra từ
`deepseek-tui`; đổi tên khi cộng đồng cần nhiều provider hơn một.

[English](README.md) · [简体中文](README.zh-CN.md) · [日本語](README.ja-JP.md) · [한국어](README.ko-KR.md) · [Español](README.es-419.md) · [Português](README.pt-BR.md) · [codewhale.net](https://codewhale.net/) · [Docs](docs) · [Changelog](CHANGELOG.md)

[![CI](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml/badge.svg)](https://github.com/Hmbown/CodeWhale/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/codewhale-cli?label=crates.io)](https://crates.io/crates/codewhale-cli)
[![npm](https://img.shields.io/npm/v/codewhale?label=npm)](https://www.npmjs.com/package/codewhale)

![Codewhale chạy trong terminal](assets/screenshot.png)

## Cài đặt

```bash
npm install -g codewhale
```

Cargo, Docker, Nix, Scoop, archive dựng sẵn, Android/Termux, và một mirror CNB
cho người dùng không truy cập được GitHub đều được hướng dẫn trong
[docs/INSTALL.md](docs/INSTALL.md). Chuyển từ `deepseek-tui` sang? Cấu hình và
session của bạn được giữ nguyên — xem [docs/REBRAND.md](docs/REBRAND.md).

## Sử dụng

```bash
codewhale auth set --provider deepseek   # or export ANTHROPIC_API_KEY, etc.
codewhale                                # open the TUI
codewhale exec "fix the failing test"    # headless
codewhale web                            # local browser client on 127.0.0.1
```

Trong TUI: `/model` đổi provider và model cùng lúc, `/fleet` chạy một đội
worker, và `/restore` hoàn tác một lượt. Khi vùng soạn thảo đang rảnh, `Tab`
chuyển vòng qua Plan / Act / Operate và `Shift+Tab` chuyển vòng qua tư thế
quyền Ask / Auto-Review / Full Access. `!` chạy một lệnh shell qua đường phê
duyệt bình thường.

## Tìm hiểu thêm

- [docs/PROVIDERS.md](docs/PROVIDERS.md) — mọi route provider: dịch vụ,
  gateway và cục bộ
- [docs/FLEET.md](docs/FLEET.md) — fleet, sổ cái và resume
- [docs/CONFIGURATION.md](docs/CONFIGURATION.md) — `config.toml`, hook và
  constitution
- [docs/WEB.md](docs/WEB.md) — trình duyệt nhúng chỉ chạy trên loopback và
  ranh giới xác thực dùng một lần

Mọi thứ còn lại — chế độ, phím tắt, chi tiết sandbox, MCP, runtime API, kiến
trúc — nằm trong [docs](docs) và trên [codewhale.net](https://codewhale.net/).

## Đóng góp

Issue, PR, các bước tái hiện lỗi và yêu cầu tính năng đều được chào đón. Khi
một PR không thể merge nguyên trạng, maintainer sẽ harvest phần dùng được và
tác giả vẫn được ghi công — trong commit, trong changelog và trong
[docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md). Thiếu một provider bạn dùng,
hoặc có gì đó hỏng trên máy của bạn? Báo cho chúng tôi biết.

- [Issue đang mở](https://github.com/Hmbown/CodeWhale/issues) — những đóng góp
  đầu tiên phù hợp nằm ở đây
- [CONTRIBUTING.md](CONTRIBUTING.md) — thiết lập môi trường dev và quy trình PR
- [docs/CONTRIBUTORS.md](docs/CONTRIBUTORS.md) — tất cả những người đã góp
  phần định hình dự án
- [Buy me a coffee](https://www.buymeacoffee.com/hmbown)

Cảm ơn [DeepSeek](https://github.com/deepseek-ai) vì các model và sự hỗ trợ đã
khởi đầu dự án, [DataWhale](https://github.com/datawhalechina) 🐋 vì đã chào
đón chúng tôi vào đại gia đình Whale Brother, và
[OpenWarp](https://github.com/zerx-lab/warp) cùng
[Open Design](https://github.com/nexu-io/open-design) vì đã hợp tác xây dựng
trải nghiệm agent trên terminal.

## Giấy phép

[MIT](LICENSE). Dự án cộng đồng độc lập; không trực thuộc bất kỳ nhà cung cấp
model nào.

[![Biểu đồ Star History](https://api.star-history.com/chart?repos=Hmbown/CodeWhale&type=date&legend=top-left)](https://www.star-history.com/?repos=Hmbown%2FCodeWhale&type=date)

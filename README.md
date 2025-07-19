# 🦀 Hamze's Ultimate Rust Project Template

> ⚙️ A fully automated Rust project starter with:
>
> * `cargo-generate` scaffolding
> * Clippy + fmt + tests + CI
> * `justfile` command runner
> * Docker build/run
> * GitHub private repo creation
> * ADHD-friendly automation 🧠
>
> *Template created by **Remolab***

## 🔧 What's Included

| Feature                        | Status |
| ------------------------------ | ------ |
| Cargo project scaffolding      | ✅      |
| Linting + formatting           | ✅      |
| GitHub Actions CI              | ✅      |
| GitHub repo auto-create        | ✅      |
| Docker support                 | ✅      |
| `just` automation runner       | ✅      |
| Editor configs (VSCode, Helix) | ✅      |

## 📁 File Structure

```text
my-rust-template/
├── .github/workflows/ci.yml
├── .gitignore
├── .editorconfig
├── .vscode/settings.json
├── helix/config.toml
├── clippy.toml
├── rustfmt.toml
├── cargo-generate.toml
├── justfile
├── Dockerfile
├── docker-compose.yml (optional)
├── post_gen.sh
├── README.md
├── src/main.rs
└── Cargo.toml
```

## ✅ Summary

| Task                        | Command             |
| --------------------------- | ------------------- |
| Create project              | `rust-new my-app`   |
| Run app                     | `just run`          |
| Run all checks              | `just check`        |
| Build Docker image          | `just docker-build` |
| Run app in Docker           | `just docker-run`   |
| Push to GitHub private repo | `./post_gen.sh`     |

## 🚀 Getting Started

1. Install dependencies:
```bash
cargo install cargo-generate
cargo install just
gh auth login
```

2. Create a new project:
```bash
cargo generate --git https://github.com/yourname/rust-starter-template.git --name my-cli-tool
cd my-cli-tool
./post_gen.sh
``` 
# 🦀 Template Usage Guide

This directory is a **cargo-generate template** ready for use.

## 📋 For Template Users

To use this template to create new projects:

```bash
# Install dependencies
cargo install cargo-generate just
gh auth login

# Create a new project
cargo generate --git https://github.com/yourname/rust-starter-template.git --name my-new-project
cd my-new-project
./post_gen.sh
```

## 🔧 Template Status

### Current State ✅
- `Cargo.toml` uses `name = "names"` - Template ready
- `Dockerfile` uses `CMD ["./target/release/names"]` - Template ready
- `src/main.rs` includes Remolab ASCII header
- Full dependency stack included for comprehensive development

### Template Variables

When users run `cargo generate`, these variables will be replaced:
- `names` → User's chosen project name
- `Hamze GHALEBI <ghalebi@gmail.com>` → Can be configured in cargo-generate

### Testing the Template

To test template generation locally:
```bash
cargo generate --path . --name test-project
cd test-project
just check
just docker-build
just run --help
```

### Dependencies Included

**Runtime:**
- `anyhow` - Error handling and context
- `clap` - Command line argument parsing
- `env_logger` - Environment-based logger
- `log` - Logging facade
- `rig-core` - AI/LLM framework core
- `schemars` - JSON Schema generation
- `serde` - Serialization/deserialization
- `tokio` - Async runtime with full features
- `tracing-subscriber` - Structured logging

**Development:**
- `pretty_assertions` - Better test output

## 🎯 Ready-to-Use Features

✅ **Development Tools**: justfile, clippy, rustfmt, editorconfig  
✅ **CI/CD**: GitHub Actions workflow  
✅ **Docker**: Multi-stage build with dependencies  
✅ **CLI**: clap-based command line interface  
✅ **Async Runtime**: tokio with full features  
✅ **Error Handling**: anyhow for context-rich errors  
✅ **Serialization**: serde for JSON/YAML/etc  
✅ **Logging**: log + env_logger + tracing-subscriber  
✅ **AI/LLM Ready**: rig-core framework included  
✅ **Schema Generation**: schemars for JSON schemas  
✅ **Testing**: pretty_assertions for better test output  
✅ **Automation**: GitHub repo creation script  
✅ **Branding**: Remolab ASCII header 
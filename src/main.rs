// ╭─────────────────────────────────────╮
// │  Template created by Remolab       │
// ╰─────────────────────────────────────╯

use clap::Parser;

/// 🦀 Remolab Rust Template
///
/// This is a template project - you need to modify it for your specific use case.
/// Replace this CLI with your actual application logic.
#[derive(Parser)]
#[command(name = "names", version, about)]
struct Cli {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();
    env_logger::init();

    // Template message - replace this with your actual application logic
    println!("🦀 This is a Remolab Rust Template!");
    println!();
    println!("📝 You need to modify this template for your specific use case:");
    println!("   • Replace this main.rs with your application logic");
    println!("   • Update Cargo.toml dependencies as needed");
    println!("   • Modify the CLI structure in the Cli struct above");
    println!("   • Add your business logic and features");
    println!();
    println!("🚀 Template features included:");
    println!("   ✅ Async runtime (tokio)");
    println!("   ✅ CLI framework (clap)");
    println!("   ✅ Error handling (anyhow)");
    println!("   ✅ Serialization (serde)");
    println!("   ✅ AI/LLM support (rig-core)");
    println!("   ✅ Schema generation (schemars)");
    println!("   ✅ Structured logging (tracing)");
    println!("   ✅ Docker support");
    println!("   ✅ CI/CD pipeline");
    println!("   ✅ Development automation (just)");
    println!();
    println!("📚 Use 'just --list' to see available development commands");

    if cli.verbose > 0 {
        log::info!("Verbosity level: {}", cli.verbose);
        println!("🔧 Verbose mode enabled (level: {})", cli.verbose);
    }
}

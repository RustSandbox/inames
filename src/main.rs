use clap::Parser;
use inames::{Generator, Name};

/// iNames - Inclusive multicultural random name generator
///
/// Generates random names using Persian, Arabic, and Asian names written in Latin characters.
/// Examples: "aziz-hamze", "sakura-krishna", "rumi-chen-1234"
#[derive(Parser)]
#[command(name = "inames", version, about, long_about = None)]
struct Cli {
    /// Add a random 4-digit number to the generated name
    #[arg(short, long)]
    number: bool,

    /// Number of names to generate
    #[arg(short, long, default_value = "1")]
    amount: usize,
}

fn main() {
    let cli = Cli::parse();

    let naming = if cli.number {
        Name::Numbered
    } else {
        Name::Plain
    };

    let mut generator = Generator::with_naming(naming);

    for _ in 0..cli.amount {
        if let Some(name) = generator.generate() {
            println!("{name}");
        }
    }
}

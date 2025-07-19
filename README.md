# iNames - Inclusive Multicultural Names Generator

[![CI](https://github.com/hamzeghalebi/inames/actions/workflows/ci.yml/badge.svg)](https://github.com/hamzeghalebi/inames/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/inames.svg)](https://crates.io/crates/inames)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE-MIT)

An inclusive random name generator for Rust that creates unique names using Persian, Arabic, and Asian names written in Latin characters. 

Inspired by [fnichol/names](https://github.com/fnichol/names), this project extends the concept to include multicultural names from around the world, making it more inclusive and diverse while maintaining the same simple and effective API.

## Examples

```
aziz-hamze
sakura-krishna
rumi-chen-1234
dalil-fatima-8472
```

## Features

- Generates random combinations of multicultural names
- All names are written in Latin/English characters
- Optional 4-digit number suffix
- Includes names from:
  - Persian/Iranian culture
  - Arabic culture
  - East Asian cultures (Chinese, Japanese)
  - South Asian cultures (Indian, Buddhist)

## Installation

### Using Homebrew (macOS and Linux)

```bash
brew tap hamzeghalebi/tap
brew install inames
```

### Using Cargo

```bash
cargo install inames
```

### Pre-built Binaries

Download pre-built binaries from the [releases page](https://github.com/hamzeghalebi/inames/releases).

### From Source

```bash
git clone https://github.com/hamzeghalebi/inames.git
cd inames
cargo build --release
cargo install --path .
```

## Usage

### Command Line

Generate a single name:
```bash
inames
```

Generate multiple names:
```bash
inames --amount 5
```

Generate names with numbers:
```bash
inames --number
inames --number --amount 10
```

### As a Library

```rust
use inames::{Generator, Name};

fn main() {
    // Default generator
    let mut generator = Generator::default();
    println!("{}", generator.next().unwrap());
    
    // With numbers
    let mut generator = Generator::with_naming(Name::Numbered);
    println!("{}", generator.next().unwrap());
    
    // Custom word lists
    let adjectives = &["aziz", "sakura", "rumi"];
    let nouns = &["hamze", "krishna", "chen"];
    let mut generator = Generator::new(adjectives, nouns, Name::Plain);
    println!("{}", generator.next().unwrap());
}
```

## Name Sources

The name lists include carefully selected names from:
- Persian names (both modern and classical)
- Arabic names (transliterated to Latin script)
- Japanese names
- Chinese names
- Indian names
- Buddhist/Sanskrit terms

All names are respectfully chosen and transliterated to be easily readable in Latin characters.

## Why This Matters

Read the essay ["The Names We Choose"](ESSAY.md) for thoughts on why multicultural naming in programming is more important than it might seem.

## Development

Use `just` to see available development commands:

```bash
just --list
```

## License

MIT OR Apache-2.0
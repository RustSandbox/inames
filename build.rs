use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Read and process adjectives
    let adjectives_path = Path::new("data/adjectives.txt");
    let dest_path = Path::new(&out_dir).join("adjectives.rs");
    process_word_list(adjectives_path, &dest_path);

    // Read and process nouns
    let nouns_path = Path::new("data/nouns.txt");
    let dest_path = Path::new(&out_dir).join("nouns.rs");
    process_word_list(nouns_path, &dest_path);

    // Tell Cargo to rerun if word lists change
    println!("cargo:rerun-if-changed=data/adjectives.txt");
    println!("cargo:rerun-if-changed=data/nouns.txt");
}

fn process_word_list(input_path: &Path, output_path: &Path) {
    let file = File::open(input_path).expect("Unable to open word list file");
    let reader = BufReader::new(file);
    let mut output = File::create(output_path).expect("Unable to create output file");

    write!(&mut output, "[").unwrap();

    let mut first = true;
    for line in reader.lines() {
        let word = line.unwrap().trim().to_string();
        if !word.is_empty() {
            if !first {
                write!(&mut output, ", ").unwrap();
            }
            write!(&mut output, "\"{}\"", word).unwrap();
            first = false;
        }
    }

    write!(&mut output, "]").unwrap();
}

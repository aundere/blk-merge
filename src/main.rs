use std::fs::File;

use clap::Parser;

use crate::types::{stringify_config, BlkConfig};

mod parsers;
mod types;

/// Command line arguments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file name
    #[arg(short, long)]
    file: String,

    /// Second file to merge with
    #[arg(short, long)]
    with: String,

    /// Output file name. Will be used instead of rewriting the first file
    #[arg(short, long)]
    output: Option<String>,

    /// Dry run mode
    #[arg(short, long)]
    dry_run: bool,

    /// Use a merging policy file
    #[arg(short = 'p', long)]
    use_policy: Option<String>,
}

/// Reads a file and parses it into a BlkConfig
fn read_and_parse(filename: &str) -> BlkConfig {
    let content = std::fs::read_to_string(filename)
        .expect("Failed to read file");

    parsers::blk::parse_config(&content)
        .expect("Failed to parse config").1
}

/// Main function
fn main() {
    let args = Args::parse();

    let first_config = read_and_parse(&args.file);
    let second_config = read_and_parse(&args.with);

    // TODO: merge two configs

    let merged_config = first_config; // Placeholder for merged config

    if !args.dry_run {
        let output_file_name = args.output.unwrap_or_else(|| args.file);

        let mut output_file = File::create(&output_file_name)
            .expect("Failed to create output file");

        stringify_config(&merged_config, &mut output_file)
            .expect("Failed to write output");
    }
}

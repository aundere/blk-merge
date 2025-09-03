use std::io::stdout;

use crate::types::stringify_config;

mod parser;
mod types;

fn main() {
    let file_name = std::env::args().nth(1)
        .expect("Please provide a file name as the first argument");

    let output_file_name = std::env::args().nth(2)
        .expect("Please provide an output file name as the second argument");

    let file = std::fs::read(file_name)
        .expect("Unable to read the file");

    let output_file = std::fs::File::create(output_file_name)
        .expect("Unable to create the output file");
    let mut output_stream = std::io::BufWriter::new(output_file);

    let content = String::from_utf8(file)
        .expect("Unable to convert file to string");

    let result = parser::parse_config(&content);

    match result {
        Ok((remaining, config)) => {
            stringify_config(&config, &mut stdout()).expect("Failed to write output");
            stringify_config(&config, &mut output_stream).expect("Failed to write output");

            if !remaining.is_empty() {
                println!("Warning: Unparsed content remaining: {}", remaining);
            }
        }
        Err(err) => eprintln!("Failed to parse config: {:?}", err),
    }
}

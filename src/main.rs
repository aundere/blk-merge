mod parser;
mod types;

fn main() {
    let file_name = std::env::args().nth(1)
        .expect("Please provide a file name as the first argument");

    let file = std::fs::read(file_name)
        .expect("Unable to read the file");

    let content = String::from_utf8(file)
        .expect("Unable to convert file to string");

    let result = parser::parse_config(&content);

    match result {
        Ok((remaining, config)) => {
            println!("Parsed config: {:#?}", config);
            if !remaining.trim().is_empty() {
                eprintln!("Warning: Unparsed content remaining: {}", remaining);
            }
        }
        Err(err) => eprintln!("Failed to parse config: {:?}", err),
    }
}

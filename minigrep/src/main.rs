use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filepath) = parse_config(&args);

    let contents = fs::read_to_string(filepath).expect("Should be able to read the file");

    println!("Text:\n{contents}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filepath = &args[2];
    (query, filepath)
}

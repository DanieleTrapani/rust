use minigrep::Config;
use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    let config = Config::build(env::args()).unwrap_or_else(|e| {
        println!("Problem parsing arguments: {e}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filepath);
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

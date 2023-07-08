use std::{env, process};
use minigrep::Config;

fn main() {

    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    println!("Searching for {}", config.query);
    println!("In file {}: \n", config.filename); 

    if let Err(e) = minigrep::run(config) {
        eprintln!("File was not found: {e}");
        process::exit(1)
    }
    
}

use std::env;
use std::process;

use rust_grep::Config;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = Config::build(&arguments).unwrap_or_else(|err| {
        eprintln!("Problem with parsing arguments {err}");
        process::exit(1);
    });

    if let Err(e) = rust_grep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

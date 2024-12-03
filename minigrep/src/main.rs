// On top of the requirement from the rust programming language book
// we will add a few grep fonctionnality. As a reminder here is grep usage
// Usage: minigrep [OPTION]... PATTERNS [FILE]...
// Unlike grep we won't do options here
// but we will implement reading from stdin like grep
// and we will also handle multi-file pattern
// it would also be nice to have a usage
// (Maybe if the mood is there we'll do --help option at the end)

use minigrep::Config;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    };
    if let Err(err) = minigrep::run(config) {
        eprintln!("Oops! An error occured: {err}");
        process::exit(1);
    };
}

use minigrep::Config;
use std::{env, process};

fn main() {
    let config = match Config::new(env::args()) {
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

// On top of the requirement from the rust programming language book
// we will add a few grep fonctionnality. As a reminder here is grep usage
// Usage: minigrep [OPTION]... PATTERNS [FILE]...
// Unlike grep we won't do options here
// but we will implement reading from stdin like grep
// and we will also handle multi-file pattern
// it would also be nice to have a usage
// (Maybe if the mood is there we'll do --help option at the end)

struct Config {
    pattern: String,
    path_list: Vec<String>,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        let pattern = match args.get(1) {
            Some(pattern) => pattern,
            _ => return Err("this is where the usage should go"),
        };
        let path_list = &args[2..];
        Ok(Config {
            pattern: pattern.to_string(),
            path_list: path_list.to_vec(),
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    if config.path_list.len() > 0 {
        println!(
            "Searching for \"{}\" in files {:?}",
            config.pattern, config.path_list
        );
        for path in config.path_list {
            let contents = std::fs::read_to_string(&path)?;
            println!("File {path} contains test:\n{contents}");
        }
    } else {
        println!("In the end this will search in stdin");
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = match Config::new(&args) {
        Ok(config) => config,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };
    if let Err(err) = run(config) {
        eprintln!("Oops! An error occured: {err}");
        std::process::exit(1);
    };
}

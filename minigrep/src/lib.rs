// On top of the requirement from the rust programming language book
// we will add a few grep fonctionnality. As a reminder here is grep usage
// Usage: minigrep [OPTION]... PATTERNS [FILE]...
// Unlike grep we won't do options here
// - [x] we will implement reading from stdin like grep
// - [x] we will also handle multi-file pattern
// - [x] it would also be nice to have a usage

pub struct Config {
    pattern: String,
    path_list: Vec<String>,
    ignore_case: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let pattern = match args.nth(1) {
            Some(pattern) => pattern,
            _ => return Err("Usage: minigrep PATTERNS [FILE]..."),
        };
        let path_list: Vec<String> = args.skip(2).collect();
        Ok(Config {
            pattern,
            path_list,
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn search_case_insensitive<'a>(pattern: &str, file_content: &'a str) -> Vec<&'a str> {
    file_content
        .lines()
        .filter(|line| line.to_lowercase().contains(&pattern.to_lowercase()))
        .collect()
}

fn search<'a>(pattern: &str, file_content: &'a str) -> Vec<&'a str> {
    file_content
        .lines()
        .filter(|line| line.contains(pattern))
        .collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let search_fn = if config.ignore_case {
        search_case_insensitive
    } else {
        search
    };
    if config.path_list.len() > 0 {
        println!(
            "Searching for \"{}\" in files {:?}",
            config.pattern, config.path_list
        );
        for path in &config.path_list {
            let content = std::fs::read_to_string(&path)?;
            for line in search_fn(&config.pattern, &content) {
                if config.path_list.len() > 1 {
                    print!("{path}:");
                }
                println!("{line}");
            }
        }
    } else {
        let stdin_handle = std::io::stdin();
        let mut buff = String::new();
        loop {
            let _ = stdin_handle.read_line(&mut buff)?;
            for line in search_fn(&config.pattern, &buff) {
                println!("{line}")
            }
            buff.clear();
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_one_result() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(pattern, contents));
    }

    #[test]
    fn search_many_result() {
        let pattern = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Productive!
Fast!
Safe!
            ";
        assert_eq!(
            vec!["safe, fast, productive.", "Productive!"],
            search(pattern, contents)
        );
    }

    #[test]
    fn search_case_not_found() {
        let pattern = "DuCt";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(0, search(pattern, contents).len());
    }

    #[test]
    fn search_case_found() {
        let pattern = "DuCt";
        let contents = "\
Rust:
safe, fast, prodUcTive.
Pick three.";
        assert_eq!(
            vec!["safe, fast, prodUcTive."],
            search_case_insensitive(pattern, contents)
        );
    }
}

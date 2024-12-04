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
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        let pattern = match args.get(1) {
            Some(pattern) => pattern,
            _ => return Err("Usage: minigrep PATTERNS [FILE]..."),
        };
        let path_list = &args[2..];
        Ok(Config {
            pattern: pattern.to_string(),
            path_list: path_list.to_vec(),
            ignore_case: std::env::var("IGNORE_CASE").is_ok(),
        })
    }
}

fn search_case_insensitive<'a>(pattern: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut lines_found: Vec<&str> = Vec::new();
    for line in file_content.lines() {
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            lines_found.push(line);
        }
    }
    lines_found
}

fn search<'a>(pattern: &str, file_content: &'a str) -> Vec<&'a str> {
    let mut lines_found: Vec<&str> = Vec::new();
    for line in file_content.lines() {
        if line.contains(pattern) {
            lines_found.push(line);
        }
    }
    lines_found
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
            let res = search_fn(&config.pattern, &buff);
            if res.len() > 0 {
                println!("{}", &res[0])
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

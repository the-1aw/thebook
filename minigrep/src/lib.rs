pub struct Config {
    pattern: String,
    path_list: Vec<String>,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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
    if config.path_list.len() > 0 {
        println!(
            "Searching for \"{}\" in files {:?}",
            config.pattern, config.path_list
        );
        for path in &config.path_list {
            let contents = std::fs::read_to_string(&path)?;
            for line in search(&config.pattern, &contents) {
                if config.path_list.len() > 1 {
                    print!("{path}:");
                }
                println!("{line}");
            }
        }
    } else {
        println!("In the end this will search in stdin");
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
}

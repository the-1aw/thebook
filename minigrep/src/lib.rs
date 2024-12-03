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

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
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

#[cfg(test)]
mod tests {
    // use super::*;
}

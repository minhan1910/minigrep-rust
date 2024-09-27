use std::error::Error;
use std::{env, fs};

#[derive(PartialEq)]
#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_two_configs_should_be_equal() {        
        let args1: Vec<String> = vec!["ignore_element_1".to_string(), "query".to_string(), "file_path".to_string()];
        let config1 = Config::build(&args1).unwrap();

        let args2: Vec<String> = vec!["ignore_element_2".to_string(), "query".to_string(), "file_path".to_string()];
        let config2 = Config::build(&args2).unwrap();

        assert_eq!(config1, config2);
    }

    #[test]
    fn build_config_should_be_correct() {        
        let query: String = "query".to_string();
        let file_path = "file_path".to_string();

        let args1: Vec<String> = vec!["ignore_element_1".to_string(), query.clone(), file_path.clone()];
        let config1 = Config::build(&args1).unwrap();

        assert_eq!(config1.query, query);
        assert_eq!(config1.file_path, file_path);
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

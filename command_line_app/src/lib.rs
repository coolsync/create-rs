use std::error::Error;
use std::{fs, env};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("No enogh arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IgnoreCase").is_ok();

        Ok(Config { query, file_path, ignore_case })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    // println!("results\n{:?}", contents.unwrap());
    
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

// For now, just know that Box<dyn Error> means the function will return a type
// that implements the Error trait,
// but we don’t have to specify what particular type the return value will be.
// fn parse_config(args: &[String]) -> Result<Config, &'static str> {
//     if args.len() < 3 {
//         return Err("No enogh arguments!");
//     }
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//     Ok(Config { query, file_path })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

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

fn search_case_insensitive2<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.trim().to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
        println!("{}", line);
        // println!("{}", &query.to_lowercase());
    }
    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.trim().to_lowercase().contains(query) {
            results.push(line);
        }
    }
    results
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // vec![]
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }
    results
}

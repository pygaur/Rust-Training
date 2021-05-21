use std::fs;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

// fn parse_config(args: &Vec<String>) -> (String, String) {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     return (query, filename);
// }

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    let mut results = vec!();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line)
        }
    }

    results
}

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

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }
}
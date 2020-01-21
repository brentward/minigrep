use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let mut args = Vec::from(args);

        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if case_sensitive {
            for (i, arg) in args.iter().enumerate() {
                if arg == &"-i".to_string() || arg == &"--ignore-case".to_string() {
                    case_sensitive = false;
                    args.remove(i);
                    // case_set_by_arg = true;
                    // index = i;
                    break;
                }
            }
        }
    
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename, case_sensitive })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
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
    fn new_config() {
        let config = Config::new(&[String::from(""), String::from("query"), String::from("filename")]).unwrap();
        assert_eq!(String::from("query"), config.query);
        assert_eq!(String::from("filename"), config.filename);
    }
    
    #[test]
    #[should_panic]
    fn not_enough_args_panics() {
        Config::new(&[String::from("")]).unwrap();
    }

    #[test]
    fn run_invalid_file() {
        let config = Config::new(&[String::from(""), String::from("query"), String::from("filename")]).unwrap();
        let error = run(config);
        assert!(error.is_err());
    }
    
   #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
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
use std::{env, error::Error, fs, process};
pub struct Config {
    query: String,
    path: String,
    ignore_case: bool,
}
impl Config {
    pub fn build(args: &[String]) -> Self {
        Config::new(args).unwrap_or_else(|err| {
            eprintln!("参数错误：{err}");
            process::exit(1);
        })
    }

    fn new(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            Err("需要至少 3 个参数")
        } else {
            Ok(Config {
                query: args[1].clone(),
                path: args[2].clone(),
                ignore_case: env::var("ignore_case").is_ok(),
            })
        }
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.path)?;
    if config.ignore_case {
        for line in search_case_insensitive(&config.query, &content) {
            println!("{line}");
        }
    } else {
        for line in search_case_sensitive(&config.query, &content) {
            println!("{line}");
        }
    }
    Ok(())
}
pub fn search_case_sensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in content.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}
pub fn search_case_insensitive<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let query = &query.to_lowercase();
    let mut result = Vec::new();
    for line in content.lines() {
        if line.to_lowercase().contains(query) {
            result.push(line);
        }
    }
    result
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_insensitive() {
        let query = "oh";
        let content = "\
你好！
Hello, thank you!
Oh, hello!
床前明月光，疑是地上霜。";
        assert_eq!(vec!["Oh, hello!"], search_case_insensitive(query, content));
    }
    #[test]
    fn case_sensitive() {
        let query = "床前明月光，";
        let content = "\
你好！
Hello, thank you!
Oh, hello!
床前明月光，疑是地上霜。";
        assert_eq!(
            vec!["床前明月光，疑是地上霜。"],
            search_case_sensitive(query, content)
        );
        let query = "Hello";
        assert_eq!(
            vec!["Hello, thank you!"],
            search_case_sensitive(query, content)
        );
    }
}

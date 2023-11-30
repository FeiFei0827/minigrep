use std::error::Error;
use std::fs;
use std::env;
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(v:&[String]) -> Result<Config, &'static str> {
        if v.len() < 3 {
            return Err("not enough arguments");
        }
        let query = v[1].clone();
        let file_path = v[2].clone();

        let ignore_case_flag = env::var("IGNORE_CASE").ok();
        let ignore_case = match ignore_case_flag.as_ref().map(String::as_ref) {
            None => false,
            Some("0") => false,
            Some(_) => true
        };

        Ok(Config { query, file_path, ignore_case })
    }
}

// pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
//     let contents = fs::read_to_string(config.file_path)?;
// // ?运算符用于将可能的错误传播给调用者进行处理。如果读取文件出现错误，函数将立即返回错误结果并终止执行。
//     println!("With text:\n{contents}");
//
//     Ok(())
//     //如果函数执行成功并成功打印文件内容，那么将返回一个表示成功的 Result 对象，其中结果为空。
// }
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

// fn run(config: Config) {
//     let contents = fs::read_to_string(config.file_path)
//         .expect("Should have been able to read the file");
//
//     println!("With text:\n{contents}");
// }
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}
pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> Vec<&'a str> {
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
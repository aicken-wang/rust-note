use std::fs;
use std::error::Error;
use std::env;
pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &content)
    }else {
        search_case_insensitive(&config.query, &content)
    };
    for line in results {
        println!("{}",line);
    }
    //println!("{:#?},{:?}",content,config.query);
    Ok(())

}
//#[derive(Debug)]
pub struct Config {
   pub query: String, // 查找的关键字
   pub filename: String, // 文档的路径
   pub case_sensitive: bool, //是否区分大小写
}
impl Config {
   pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();// 创建副本将副本的所有权给Config
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Config{query, filename, case_sensitive})
    }
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
// 忽略大小写
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();

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
    fn one_result() {
        let query = "duct";
        let contents ="\
Rust:
safe fast,productive.
pick three.";
        assert_eq!(vec!["safe fast,productive."], search(query,contents))
    }
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents ="\
Rust:
safe,fast,
Pick three.
Trust me.";
        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query,contents))
    }
}

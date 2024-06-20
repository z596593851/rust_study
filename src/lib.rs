use std::any::Any;
use std::error::Error;
use std::fs;
pub mod some_thing;
mod io_demo;
mod exercise;
mod leetcode;
mod async_demo;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string")
        };
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.file_path)?;
    let mut result = search(&config.query, &content);
    for line in result {
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    content
        .lines()
        .filter(|line| line.contains(query))
        .collect()

}
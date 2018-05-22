extern crate regex;
extern crate colored;


pub struct Config {
  filename: String,
  query: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {

    return if args.len() < 2 {
      Err("Not enough arguments supplied!")
    } else {
      Ok(Config { query: args[0].clone(), filename: args[1].clone() })
    }

  }
}


pub mod main {

  use super::*;
  use std::error::Error;
  use std::fs::File;
  use std::io::prelude::*;
  use regex::RegexBuilder;
  use colored::*;

  static REGEX_SIZE_LIMIT: usize = 1_000_000; // Bytes


  fn read_file(filename: &str) -> Result<String, Box<Error>> {
    let mut contents = String::new();
    let mut file = File::open(filename)?;
    file.read_to_string(&mut contents)?;

    Ok(contents)
  }

  pub fn execute (contents: &str, query: &str) -> Result<Vec<String>, Box<Error>> {

    let pattern = format!("(.+({}).+)\n", query);
    let re = RegexBuilder::new(&pattern).size_limit(REGEX_SIZE_LIMIT).build()?;

    let res: Vec<String> = re.captures_iter(&contents)
      .map(|caps| {
        let outer = caps.get(1).unwrap().as_str();
        let inner = caps.get(2).unwrap().as_str();
        format!("{}", outer.replace(&inner, &format!("{}", &inner.red())))
      })
      .collect();

    Ok(res)
  }

  pub fn run(config: Config) -> Result<(), Box<Error>>{

    println!("Querying '{}'\nwith '{}'\n", config.filename, config.query);

    let contents = read_file(&config.filename)?;
    let lines = execute(&contents, &config.query)?;

    for l in lines {
      println!("{}", l);
    }

    Ok(())
  }

}


#[cfg(test)]
mod tests;

use std::fs;
use std::error::Error;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let file_contents = 
    fs::read_to_string(config.file_path)?;

  let results = if config.ignore_case {
    search_case_insensitive(config.query, &file_contents)
  } else {
    search(config.query, &file_contents)
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
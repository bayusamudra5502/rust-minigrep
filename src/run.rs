use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let buffer = BufReader::new(file);

    let line_iters = buffer.lines().map(|line| line.unwrap());
    let results = if config.ignore_case {
        filter_case_insensitive(line_iters, &config.query)
    } else {
        filter_case_sensitive(line_iters, &config.query)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub fn filter_case_sensitive<'a>(
    it: impl Iterator<Item = String> + 'a,
    query: &'a str,
) -> Box<dyn Iterator<Item = String> + 'a> {
    Box::new(it.filter(move |line| line.contains(&query)))
}

pub fn filter_case_insensitive<'a>(
    it: impl Iterator<Item = String> + 'a,
    query: &'a str,
) -> Box<dyn Iterator<Item = String> + 'a> {
    let query = query.to_lowercase();
    Box::new(it.filter(move |line| line.to_lowercase().contains(&query)))
}

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file = File::open(config.file_path)?;
    let buffer = BufReader::new(file);

    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query
    };

    let results = buffer.lines().map(|line| line.unwrap()).filter(|line| {
        if config.ignore_case {
            line.to_lowercase().contains(&query)
        } else {
            line.contains(&query)
        }
    });

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

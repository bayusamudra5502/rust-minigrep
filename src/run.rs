use std::error::Error;
use std::fs;

use crate::config::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let file_contents = fs::read_to_string(config.file_path)?;
    let query = if config.ignore_case {
        config.query.to_lowercase()
    } else {
        config.query
    };

    let results = file_contents.lines().filter(|line| {
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

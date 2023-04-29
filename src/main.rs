use std::{env, process};

use minigrep::config::Config;
use minigrep::run::run;

fn main() {
    let config = Config::parse(env::args()).unwrap_or_else(|err| {
        eprintln!("Usage : minigrep <query> <filepath>");
        eprintln!("{}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Error : {}", err);
        process::exit(-1);
    }
}

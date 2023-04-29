use std::env;

const CASE_INSENSITIVE_PARAM: &str = "--case-insensitive";
const CASE_SENSITIVE_PARAM: &str = "--case-sensitive";

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new(query: String, file_path: String, ignore_case: bool) -> Config {
        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    pub fn parse(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let mut query: Result<String, &str> = Err("query is empty");
        let mut file_path: Result<String, &str> = Err("file path is empty");
        let mut ignore_case = env::var("IGNORE_CASE").is_ok();
        let mut is_end = false;

        let mut is_query = true;
        let mut is_path = false;

        args.next();

        while !is_end {
            match args.next() {
                Some(str) => match str.as_str() {
                    CASE_INSENSITIVE_PARAM => ignore_case = true,
                    CASE_SENSITIVE_PARAM => ignore_case = false,
                    _ => {
                        if str.starts_with("-") {
                            return Err("unknown parameter");
                        }

                        if is_query {
                            query = Ok(str);
                            is_query = false;
                            is_path = true;
                        } else if is_path {
                            file_path = Ok(str);
                            is_path = false;
                        }
                    }
                },
                None => is_end = true,
            }
        }

        Ok(Config::new(query?, file_path?, ignore_case))
    }
}

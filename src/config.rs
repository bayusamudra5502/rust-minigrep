use std::env;

const CASE_INSENSITIVE_PARAM: &str = "--case-insensitive";
const CASE_SENSITIVE_PARAM: &str = "--case-sensitive";

pub struct Config<'a> {
  pub query: &'a str,
  pub file_path: &'a str,
  pub ignore_case: bool
}

impl<'a> Config<'a> {
    pub fn new(query: &'a str, file_path: &'a str, ignore_case: bool) -> Config<'a> {
      Config {
        query,
        file_path,
        ignore_case
      }
    }

    pub fn parse(args: &[String]) -> Result<Config, &str> {
      if args.len() < 3 {
        return Err("not enough arguments");
      }
    
      let mut query: Result<&str, &str> = Err("query is empty");
      let mut file_path: Result<&str, &str> = Err("file path is empty");
      let mut ignore_case = env::var("IGNORE_CASE").is_ok();
      let mut idx = 0;

      for word in args {
        if word == CASE_INSENSITIVE_PARAM {
          ignore_case = true;
          continue;
        } else if word == CASE_SENSITIVE_PARAM {
          ignore_case = false;
          continue;
        } else if idx == 1 {
            query = Ok(&word);
        } else if idx == 2 {
            file_path = Ok(word);
        }

        idx += 1;
      }
    
      Ok(Config::new(query?, file_path?, ignore_case))
    }
}

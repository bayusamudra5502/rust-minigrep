use minigrep::run::*;

#[test]
#[ignore = "deprecated"]
fn test_search() {
    let query = "duct";
    let contents = "\
  Rust:
  safe, fast, productive.
  Pick three.";

    // assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}

#[test]
#[ignore = "deprecated"]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    // assert_eq!(
    //     vec!["Rust:", "Trust me."],
    //     search_case_insensitive(query, contents)
    // );
}

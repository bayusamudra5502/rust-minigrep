use minigrep::run::*;

#[test]
fn test_search() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    let string_lines = contents.lines().map(|l| String::from(l));
    let it = filter_case_sensitive(string_lines, query);
    let collected: Vec<String> = it.collect();

    assert_eq!(vec!["safe, fast, productive."], collected);
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    let string_lines = contents.lines().map(|l| String::from(l));
    let it = filter_case_insensitive(string_lines, query);
    let collected: Vec<String> = it.collect();

    assert_eq!(vec!["Rust:", "Trust me."], collected);
}

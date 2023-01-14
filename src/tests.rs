#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents, true));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search(query, contents, false));
    }
}
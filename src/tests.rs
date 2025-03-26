#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust Duct tape.";

        assert_eq!(vec!["Safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
Safe, fast, productive.
Pick three.
Trust Duct tape.";

        assert_eq!(
            vec!["Rust:", "Trust Duct tape."],
            search_case_insensitive(query, contents)
        );
    }
}

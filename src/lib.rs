use std::fs;
use std::error::Error;

mod config;
pub use self::config::Config;

mod tests;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(config.query, &contents, config.is_case_sensitive) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a, 'b>(query: &'a str, contents: &'b str, is_case_sensitive: bool) -> Vec<&'b str> {
    let mut results = Vec::new();

    if is_case_sensitive {
        // case sensitive version
        for line in contents.lines() {
            if line.contains(query) {
                results.push(line)
            }
        }
    } else {
        // case insensitive version
        let contents_lower = contents.to_lowercase();
        let query_lower = query.to_lowercase();
        let collected_lines = contents.lines().collect::<Vec<&str>>();

        for (k, line) in contents_lower.lines().enumerate() {
            if line.contains(&query_lower) {
                results.push(collected_lines[k])
            }
        }
    }

    results
}
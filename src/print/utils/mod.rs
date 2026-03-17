#![allow(warnings)]
use regex::Regex;

pub fn extract_lines_with_numbers(line: &str) -> Result<&str, regex::Error> {
    let regex = Regex::new(r"[0-9]+").expect("Invalid regex pattern");
    if regex.is_match(line) {
        return Ok(line);
    }
    Err(regex::Error::Syntax(String::from("Pattern parsing error")))
}

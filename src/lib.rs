use std::fs::File;
use std::io::{self, BufRead};

use regex::Regex;

pub fn grep(pattern: &str, file_path: &str) -> io::Result<Vec<String>> {
    let regex = match Regex::new(pattern) {
        Ok(r) => r,
        Err(err) => return Err(io::Error::new(io::ErrorKind::InvalidInput, err)),
    };

    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut matched_lines = Vec::new();
    for (_, line) in reader.lines().enumerate() {
        let line = line?;
        if regex.is_match(&line) {
            matched_lines.push(line.trim().to_string());
        }
    }
    Ok(matched_lines)
}

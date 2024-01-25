use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

use regex::Regex;
use walkdir::WalkDir;

// MatchItem represents a match in grep searching
pub struct MatchItem {
    pub line_number: usize,
    pub line: String,
}

// MatchResult represents all matches of all files of a grep search
pub type MatchResult = std::collections::HashMap<String, Vec<MatchItem>>;

pub struct GrepOptions {
    pub ignore_case: bool,
    pub invert_match: bool,
}

pub fn grep(
    pattern: &str,
    file_path: &Path,
    options: &GrepOptions,
) -> Result<MatchResult, io::Error> {
    let real_pattern = if options.ignore_case {
        format!("(?i){}", pattern)
    } else {
        pattern.to_string()
    };
    let pattern_regex = regex::Regex::new(&real_pattern)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, format!("Regex error: {}", err)))?;

    let lines = read_file_lines(file_path)?;
    let matching_lines = if options.invert_match {
        filter_lines(&pattern_regex, &lines, false)
    } else {
        filter_lines(&pattern_regex, &lines, true)
    };

    let mut result = MatchResult::new();
    result.insert(file_path.to_string_lossy().to_string(), matching_lines);
    Ok(result)
}

pub fn grep_count(result: &MatchResult) -> usize {
    result.values().map(|v| v.len()).sum()
}

pub fn grep_recursive(
    pattern: &str,
    directory_path: &Path,
    options: &GrepOptions,
) -> Result<MatchResult, io::Error> {
    let mut results = MatchResult::new();
    for entry in WalkDir::new(directory_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            let file_path = entry.path();
            let result = grep(pattern, file_path, options)?;
            results.extend(result);
        }
    }
    Ok(results)
}

fn read_file_lines(file_path: &Path) -> Result<Vec<String>, io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    Ok(reader.lines().map_while(Result::ok).collect())
}

fn filter_lines(pattern: &Regex, lines: &[String], flag: bool) -> Vec<MatchItem> {
    lines
        .iter()
        .enumerate()
        .filter_map(|(line_number, line)| {
            if flag == pattern.is_match(line) {
                Some(MatchItem {
                    line_number: line_number + 1,
                    line: line.trim().to_string(),
                })
            } else {
                None
            }
        })
        .collect()
}

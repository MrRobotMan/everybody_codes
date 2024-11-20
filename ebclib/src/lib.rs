use std::{fmt::Display, fs::read_to_string, path::Path};

/// Gather a string of text or file name to a string.
pub fn lines<T: AsRef<Path> + Display>(path: T) -> String {
    match path.as_ref().exists() {
        false => path.to_string(),
        true => read_to_string(path).expect("Failed to open file {path}"),
    }
}

/// Read the input to a vec of strings.
pub fn read_lines<T: AsRef<Path> + Display>(file: T) -> Vec<String> {
    lines(file)
        .lines()
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// Read the input line to a vec of chars.
pub fn read_line_chars<T: AsRef<Path> + Display>(file: T) -> Vec<char> {
    lines(file).trim().chars().filter(|c| *c != '\n').collect()
}

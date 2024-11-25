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

/// Read the input line to a vec of ch bars.
pub fn read_line_chars<T: AsRef<Path> + Display>(file: T) -> Vec<char> {
    lines(file).trim().chars().filter(|c| *c != '\n').collect()
}

/// Read hte input to a grid
pub fn read_grid<T: AsRef<Path> + Display>(file: T) -> Vec<Vec<char>> {
    lines(file)
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

/// Array of 4 ordinal direction offsets.
pub const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

/// Array of 8 directional offsets
pub const ALL_DIRS: [(i64, i64); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = "..##..\n##..##\n..##..";
        let actual = read_grid(input);
        let expected = vec![
            vec!['.', '.', '#', '#', '.', '.'],
            vec!['#', '#', '.', '.', '#', '#'],
            vec!['.', '.', '#', '#', '.', '.'],
        ];
        assert_eq!(actual, expected);
    }
}

use std::{collections::HashMap, fmt::Display, fs::read_to_string, path::Path};

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
pub fn read_grid<T: AsRef<Path> + Display>(file: T) -> HashMap<(i64, i64), char> {
    lines(file)
        .trim()
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, ch)| ((row as i64, col as i64), ch))
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let input = "..##..\n##..##\n..##..";
        let actual = read_grid(input);
        let expected = HashMap::from([
            ((0, 0), '.'),
            ((0, 1), '.'),
            ((0, 2), '#'),
            ((0, 3), '#'),
            ((0, 4), '.'),
            ((0, 5), '.'),
            ((1, 0), '#'),
            ((1, 1), '#'),
            ((1, 2), '.'),
            ((1, 3), '.'),
            ((1, 4), '#'),
            ((1, 5), '#'),
            ((2, 0), '.'),
            ((2, 1), '.'),
            ((2, 2), '#'),
            ((2, 3), '#'),
            ((2, 4), '.'),
            ((2, 5), '.'),
        ]);
        assert_eq!(actual, expected);
    }
}

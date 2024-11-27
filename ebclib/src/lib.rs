use std::{fmt::Display, fs::read_to_string, ops::Deref, path::Path};
pub mod math;

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

/// Read the input to a grid
pub fn read_grid<T: AsRef<Path> + Display>(file: T) -> Vec<Vec<char>> {
    lines(file)
        .trim()
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

/// Get the midpoint(s) of an array. If the array is sorted this will get the median value(s);
/// For odd length arrays the slice will contain 1 element, for even length 2.
pub trait Median<T: Ord> {
    fn mid(&self) -> &[T]
    where
        Self: Deref<Target = [T]>,
    {
        let midpoint = self.len() / 2;
        match self.len() {
            x if x <= 2 => self.deref(), // For empty, lenth 1, and length 2 mid is the slice.
            x if x % 2 == 1 => &self[midpoint..midpoint + 1], // For odd length the midpoint is the middle.
            _ => &self[midpoint - 1..midpoint + 1], // For even length the midpoint is between two elements.
        }
    }
}

impl<T: Ord> Median<T> for Vec<T> {}
impl<T: Ord> Median<T> for &[T] {}

/// Array of 4 ordinal direction offsets. Up, Right, Down, Left
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

    #[test]
    fn test_odd_array_mid() {
        let mut odd = vec![1, 4, 3, 5, 6];
        assert_eq!(odd.mid(), [3]);
        odd.sort();
        assert_eq!(odd.mid(), [4]);
    }

    #[test]
    fn test_even_array_mid() {
        let even = [1, 2, 3, 4];
        assert_eq!(even.as_slice().mid(), [2, 3]);
    }
}

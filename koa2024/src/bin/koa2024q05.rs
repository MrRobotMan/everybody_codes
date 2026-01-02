use puzlib::read_lines;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = read_lines("koa2024/inputs/quest05.1.txt");
    println!("Part 1: {}", part_one(input));

    let input = read_lines("koa2024/inputs/quest05.2.txt");
    println!("Part 2: {}", part_two(input));

    let input = read_lines("koa2024/inputs/quest05.3.txt");
    println!("Part 3: {}", part_three(input));
}

fn part_one(input: Vec<String>) -> u64 {
    let mut dance = Dance::new(&input);
    (0..9).for_each(|idx| {
        dance.step(idx % dance.columns.len());
    });
    dance.step(9 % dance.columns.len())
}

fn part_two(input: Vec<String>) -> u64 {
    let mut dance = Dance::new(&input);
    dance.dance_until(2024)
}

fn part_three(input: Vec<String>) -> u64 {
    let mut dance = Dance::new(&input);
    dance.dance_forever()
}

#[derive(Debug)]
struct Dance {
    columns: Vec<VecDeque<u64>>,
}

impl Dance {
    fn new<S: AsRef<str>>(input: &[S]) -> Self {
        let mut columns = vec![VecDeque::new(); 4];
        for row in input {
            for (col, ch) in row.as_ref().split(' ').enumerate() {
                columns[col].push_back(ch.parse().unwrap());
            }
        }
        Self { columns }
    }

    fn step(&mut self, clapper_col: usize) -> u64 {
        let clapper = self.columns[clapper_col].pop_front().unwrap();
        let next_column = (clapper_col + 1) % self.columns.len();
        let next_column_len = 2 * self.columns[next_column].len();
        let remainder = (clapper as usize - 1) % next_column_len;
        let position = remainder.min(next_column_len - remainder);
        self.columns[next_column].insert(position, clapper);
        self.columns
            .iter()
            .fold(0, |acc, col| acc * 10_u64.pow(col[0].ilog10() + 1) + col[0])
    }

    fn dance_until(&mut self, target: u64) -> u64 {
        let mut count = HashMap::new();
        let mut round = 0;
        loop {
            let result = self.step(round % self.columns.len());
            count.entry(result).and_modify(|v| *v += 1).or_insert(1);
            round += 1;
            if count[&result] == target {
                return round as u64 * result;
            }
        }
    }

    fn dance_forever(&mut self) -> u64 {
        let mut states = HashSet::new();
        let mut round = 0;
        while states.insert(self.columns.clone()) {
            self.step(round % self.columns.len());
            round += 1;
        }
        states.iter().map(|v| calc(v)).max().unwrap()
    }
}

fn calc(value: &[VecDeque<u64>]) -> u64 {
    value
        .iter()
        .fold(0_u64, |acc, v| acc * 10_u64.pow(v[0].ilog10() + 1) + v[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut dance = Dance::new(&["2 3 4 5", "3 4 5 2", "4 5 2 3", "5 2 3 4"]);
        let expected = [3345, 3245, 3255, 3252, 4252, 4452, 4422, 4423, 2423, 2323];
        (0..10).for_each(|idx| {
            assert_eq!(expected[idx], dance.step(idx % dance.columns.len()));
        });
    }

    #[test]
    fn test_two() {
        let mut dance = Dance::new(&["2 3 4 5", "6 7 8 9"]);
        assert_eq!(50877075, dance.dance_until(2024));
    }

    #[test]
    fn test_three() {
        let mut dance = Dance::new(&["2 3 4 5", "6 7 8 9"]);
        assert_eq!(6584, dance.dance_forever());
    }
}

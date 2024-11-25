use ebclib::read_lines;
use std::collections::{HashMap, VecDeque};

fn main() {
    let input = read_lines("ebc2024/inputs/quest05.1.txt");
    println!("Part 1: {}", part_one(input));

    let input = read_lines("ebc2024/inputs/quest05.2.txt");
    println!("Part 2: {}", part_two(input));

    let _input = read_lines("ebc2024/inputs/quest05.3.txt");
    println!("Part 3: {}", part_three());
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
    dance.dance_forever(2024)
}

fn part_three() -> String {
    "Unsolved".into()
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
        let mut mod_clapper = clapper as usize;
        let next_column = (clapper_col + 1) % self.columns.len();
        let next_column_len = self.columns[next_column].len();
        while mod_clapper > 0 {
            if mod_clapper <= next_column_len {
                self.columns[next_column].insert(mod_clapper - 1, clapper);
                break;
            } else if mod_clapper < next_column_len * 2 {
                self.columns[next_column].insert(next_column_len * 2 - mod_clapper + 1, clapper);
                break;
            } else {
                mod_clapper -= next_column_len * 2;
            }
        }
        let mut res = 0;
        for col in self.columns.iter() {
            res = res * 10_u64.pow(col[0].ilog10() + 1) + col[0]
        }
        res
    }

    fn dance_forever(&mut self, target: u64) -> u64 {
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
        assert_eq!(50877075, dance.dance_forever(2024));
    }
}

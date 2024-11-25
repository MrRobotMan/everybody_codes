use ebclib::read_lines;
use std::collections::VecDeque;

fn main() {
    let input = read_lines("ebc2024/inputs/quest05.1.txt");
    println!("Part 1: {}", part_one(input));

    let _input = read_lines("ebc2024/inputs/quest05.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest05.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(input: Vec<String>) -> String {
    let mut dance = Dance::new(&input);
    (0..9).for_each(|idx| {
        dance.step(idx % dance.columns.len());
    });
    dance.step(9 % dance.columns.len())
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

#[derive(Debug)]
struct Dance {
    columns: Vec<VecDeque<u8>>,
}

impl Dance {
    fn new<S: AsRef<str>>(input: &[S]) -> Self {
        let lines = input
            .iter()
            .map(|l| l.as_ref().replace(' ', ""))
            .collect::<Vec<_>>();
        let mut columns = vec![VecDeque::new(); lines[0].len()];
        for row in lines {
            for (col, ch) in row.chars().enumerate() {
                columns[col].push_back(ch as u8 - b'0');
            }
        }
        Self { columns }
    }

    fn step(&mut self, clapper_col: usize) -> String {
        let clapper = self.columns[clapper_col].pop_front().unwrap();
        let mut mod_clapper = clapper as usize;
        let mut next_column = (clapper_col + 1) % self.columns.len();
        while mod_clapper > 0 {
            let next_column_len = self.columns[next_column].len();
            if mod_clapper <= next_column_len {
                self.columns[next_column].insert(mod_clapper - 1, clapper);
                break;
            } else if mod_clapper < next_column_len * 2 {
                self.columns[next_column].insert(next_column_len * 2 - mod_clapper + 1, clapper);
                break;
            } else {
                mod_clapper -= next_column_len * 2;
                next_column += 1
            }
        }
        let mut res = String::new();
        for col in self.columns.iter() {
            res.push((col[0] + b'0') as char)
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mut dance = Dance::new(&["2 3 4 5", "3 4 5 2", "4 5 2 3", "5 2 3 4"]);
        let expected = [
            "3345", "3245", "3255", "3252", "4252", "4452", "4422", "4423", "2423", "2323",
        ];
        (0..10).for_each(|idx| {
            assert_eq!(expected[idx], dance.step(idx % dance.columns.len()));
        });
    }

    #[test]
    fn test_bigger_than_col() {
        let mut dance = Dance::new(&["6 3 4 5", "5 2 3 4"]);
        assert_eq!("5345", dance.step(0))
    }
}

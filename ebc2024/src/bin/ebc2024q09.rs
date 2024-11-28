use ebclib::{read_lines, read_numbers};

const STAMPS: [u64; 4] = [1, 3, 5, 10];

fn main() {
    let input = read_numbers("ebc2024/inputs/quest09.1.txt");
    println!("Part 1: {}", part_one(&input));

    let _input = read_lines("ebc2024/inputs/quest09.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest09.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(sparkballs: &[u64]) -> u64 {
    sparkballs.iter().map(|s| get_beetle_count(*s)).sum()
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn get_beetle_count(mut sparkball: u64) -> u64 {
    let mut total = 0;
    for stamp in STAMPS.iter().rev() {
        if *stamp <= sparkball {
            let count = sparkball / stamp;
            total += count;
            sparkball -= count * stamp;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let sparkballs = [2, 4, 7, 16];
        let counts = sparkballs
            .iter()
            .map(|s| get_beetle_count(*s))
            .collect::<Vec<_>>();
        assert_eq!([2, 2, 3, 3], counts.as_slice());
        assert_eq!(10, counts.iter().sum::<u64>());
    }
}

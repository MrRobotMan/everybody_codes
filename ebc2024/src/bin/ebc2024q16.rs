use std::collections::HashMap;

use puzlib::{lcm, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest16.1.txt");
    let wheels = parse_input(input);
    println!("Part 1: {}", part_one(wheels, 100));

    let input = read_lines("ebc2024/inputs/quest16.2.txt");
    let wheels = parse_input(input);
    println!("Part 2: {}", part_two(wheels, 202420242024));

    let _input = read_lines("ebc2024/inputs/quest16.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(wheels: Vec<Wheel>, pulls: usize) -> String {
    let wh = wheels
        .iter()
        .map(|wheel| wheel.position(pulls).iter().collect::<String>())
        .collect::<Vec<_>>();
    wh.join(" ")
}

fn part_two(wheels: Vec<Wheel>, pulls: usize) -> usize {
    let cycle = wheels
        .iter()
        .fold(1, |acc, wheel| lcm(acc, wheel.sequence.len()));

    let mut coins = 0;
    for pull in 1..=cycle {
        coins += score(wheels.iter().map(|wheel| wheel.position(pull)));
    }
    coins *= pulls / cycle;
    let rem = pulls % cycle;
    for pull in 0..rem {
        coins += score(wheels.iter().map(|wheel| wheel.position(pull)));
    }

    coins
}

fn part_three() -> String {
    "Unsolved".into()
}

fn score<T: Iterator<Item = [char; 3]>>(wheels: T) -> usize {
    let mut icons: HashMap<char, usize> = HashMap::new();
    for wheel in wheels {
        let v = icons.entry(wheel[0]).or_insert(0);
        *v += 1;
        let v = icons.entry(wheel[2]).or_insert(0);
        *v += 1;
    }
    icons.into_values().map(|v| v.saturating_sub(2)).sum()
}

#[derive(Debug, Default)]
struct Wheel {
    steps: usize,
    sequence: Vec<[char; 3]>,
}

impl Wheel {
    fn position(&self, pull: usize) -> [char; 3] {
        self.sequence[(self.steps * pull) % self.sequence.len()]
    }
}

fn parse_input(input: Vec<String>) -> Vec<Wheel> {
    let mut wheels = input[0]
        .split(',')
        .map(|steps| Wheel {
            steps: steps.parse().unwrap(),
            ..Default::default()
        })
        .collect::<Vec<_>>();
    for row in &input[1..] {
        let chars = row.chars().collect::<Vec<_>>();
        for (wheel, idx) in (0..chars.len()).step_by(4).enumerate() {
            if chars[idx] != ' ' {
                wheels[wheel]
                    .sequence
                    .push([chars[idx], chars[idx + 1], chars[idx + 2]]);
            };
        }
    }
    wheels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = ">.- -.- ^,-";
        let wheels = parse_input(read_lines(
            "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
        ));
        let actual = part_one(wheels, 100);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two() {
        let expected = 280014668134;
        let wheels = parse_input(read_lines(
            "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
        ));
        let actual = part_two(wheels, 202420242024);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_score() {
        let expected = 4;
        let actual = score([['^', '_', '^'], ['^', '_', '^'], ['^', '_', '^']].into_iter());
        assert_eq!(expected, actual);
    }
}

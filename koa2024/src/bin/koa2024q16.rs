use std::collections::HashMap;

use puzlib::{lcm, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest16.1.txt");
    let wheels = parse_input(input);
    println!("Part 1: {}", part_one(wheels, 100));

    let input = read_lines("ebc2024/inputs/quest16.2.txt");
    let wheels = parse_input(input);
    println!("Part 2: {}", part_two(wheels, 202420242024));

    let input = read_lines("ebc2024/inputs/quest16.3.txt");
    let wheels = parse_input(input);
    println!("Part 3: {}", part_three(wheels, 256));
}

fn part_one(wheels: Vec<Wheel>, pulls: usize) -> String {
    let wh = wheels
        .iter()
        .map(|wheel| wheel.position(pulls, 0, 0).iter().collect::<String>())
        .collect::<Vec<_>>();
    wh.join(" ")
}

fn part_two(wheels: Vec<Wheel>, pulls: usize) -> usize {
    let cycle = wheels
        .iter()
        .fold(1, |acc, wheel| lcm(acc, wheel.sequence.len()));

    let mut coins = 0;
    for pull in 1..=cycle {
        coins += score(wheels.iter().map(|wheel| wheel.position(pull, 0, 0)));
    }
    coins *= pulls / cycle;
    let rem = pulls % cycle;
    for pull in 0..rem {
        coins += score(wheels.iter().map(|wheel| wheel.position(pull, 0, 0)));
    }

    coins
}

fn part_three(wheels: Vec<Wheel>, pulls: usize) -> String {
    let mid = pulls + 1;
    let mut cur_min = vec![usize::MAX; 2 * mid + 1];
    let mut next_min = vec![usize::MAX; 2 * mid + 1];
    let mut cur_max = vec![0; 2 * mid + 1];
    let mut next_max = vec![0; 2 * mid + 1];
    cur_min[mid] = 0;

    for pull in 1..=pulls {
        for value in (mid - pull)..=(mid + pull) {
            let backward = mid.saturating_sub(value);
            let foreward = value.saturating_sub(mid);
            let score = score(
                wheels
                    .iter()
                    .map(|wheel| wheel.position(pull, backward, foreward)),
            );
            next_min[value] = score + cur_min[value - 1..=value + 1].iter().min().unwrap();
            next_max[value] = score + cur_max[value - 1..=value + 1].iter().max().unwrap();
        }
        (cur_min, next_min) = (next_min, cur_min);
        (cur_max, next_max) = (next_max, cur_max);
    }

    format!(
        "{} {}",
        cur_max.iter().max().unwrap(),
        cur_min.iter().min().unwrap()
    )
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
    fn position(&self, pull: usize, backward: usize, foreward: usize) -> [char; 3] {
        let ct = self.sequence.len();
        self.sequence[(self.steps * pull + foreward + (ct - backward % ct)) % ct]
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
    fn test_three() {
        let expected = "627 128";
        let wheels = parse_input(read_lines(
            "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- ^.^
    -.^ >.<
    >.>",
        ));
        let actual = part_three(wheels, 256);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_score() {
        let expected = 4;
        let actual = score([['^', '_', '^'], ['^', '_', '^'], ['^', '_', '^']].into_iter());
        assert_eq!(expected, actual);
    }
}

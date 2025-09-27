use std::collections::HashMap;

use puzlib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest16.1.txt");
    let wheels = parse_input(input);
    println!("Part 1: {}", part_one(wheels));

    let _input = read_lines("ebc2024/inputs/quest16.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest16.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(wheels: Vec<Wheel>) -> String {
    let wh = wheels
        .iter()
        .map(|wheel| {
            wheel.sequence[(100 * wheel.steps) % wheel.sequence.len()]
                .iter()
                .collect::<String>()
        })
        .collect::<Vec<_>>();
    wh.join(" ")
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn score<T: Iterator<Item = [char; 3]>>(wheels: T) -> usize {
    let mut icons: HashMap<char, usize> = HashMap::new();
    for wheel in wheels {
        println!("{wheel:?}");
        for c in wheel {
            let v = icons.entry(c).or_insert(0);
            *v += 1;
        }
    }
    icons.into_values().map(|v| v.saturating_sub(2)).sum()
}

#[derive(Debug, Default)]
struct Wheel {
    steps: usize,
    sequence: Vec<[char; 3]>,
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
    fn test_parse() {
        let expected = 2;
        let wheels = parse_input(read_lines(
            "1,2,3

^_^ -.- ^,-
>.- ^_^ >.<
-_- -.- >.<
    -.^ ^_^
    >.>",
        ));
        let actual = part_one(wheels);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_score() {
        let expected = 5;
        let actual = score([['^', '_', '^'], ['^', '_', '^'], ['^', '_', '^']].into_iter());
        assert_eq!(expected, actual);
    }
}

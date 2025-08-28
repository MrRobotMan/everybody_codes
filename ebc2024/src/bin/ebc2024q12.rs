use std::cmp::Ordering;

use ebclib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest12.1.txt");
    println!("Part 1: {}", part_one(parse_input(input)));

    let input = read_lines("ebc2024/inputs/quest12.2.txt");
    println!("Part 2: {}", part_one(parse_input(input)));

    let _input = read_lines("ebc2024/inputs/quest12.3.txt");
    println!("Part 3: {}", part_three());
}

fn parse_input(lines: Vec<String>) -> Vec<(usize, usize, usize)> {
    let mut offset = 0;
    let mut res = Vec::new();
    for (row, line) in lines.iter().rev().enumerate().skip(1) {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'A' => offset = col,
                'T' => res.push((col - offset, row, 1)),

                'H' => res.push((col - offset, row, 2)),

                _ => (),
            }
        }
    }
    res.sort_by(|a, b| match b.1.cmp(&a.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        o => o,
    });
    res
}

fn score_target(target: &(usize, usize, usize)) -> usize {
    for height in 1..4 {
        let h = height as f64;
        let (x, y) = (target.0 as f64, target.1 as f64);
        let power = (x + y - h - 3.0) / 3.0 + 1.0;
        if power.fract() == 0.0 {
            return target.2 * height * power as usize;
        }
    }
    panic!("Not able to hit target {target:?}");
}

fn part_one(targets: Vec<(usize, usize, usize)>) -> usize {
    targets.iter().map(score_target).sum()
}

fn part_three() -> String {
    "Unsolved".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![(7, 2, 1), (7, 1, 1), (9, 1, 1)];
        let lines = r#".............
.C...........
.B......T....
.A......T.T..
============="#
            .lines()
            .map(|l| l.into())
            .collect();
        let actual = parse_input(lines);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_one() {
        let lines = r#".............
.C...........
.B......T....
.A......T.T..
============="#
            .lines()
            .map(|l| l.into())
            .collect();
        let targets = parse_input(lines);
        let actual = part_one(targets);
        let expected = 13;
        assert_eq!(expected, actual);
    }
}

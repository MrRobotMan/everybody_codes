use std::cmp::Ordering;

use puzlib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest12.1.txt");
    println!("Part 1: {}", stationary(parse_input(input)));

    let input = read_lines("ebc2024/inputs/quest12.2.txt");
    println!("Part 2: {}", stationary(parse_input(input)));

    let input = read_lines("ebc2024/inputs/quest12.3.txt")
        .iter()
        .filter_map(|line| {
            line.split_once(' ')
                .map(|v| (v.0.parse().unwrap(), v.1.parse().unwrap()))
        })
        .collect();
    println!("Part 3: {}", moving(input));
}

fn parse_input(lines: Vec<String>) -> Vec<(usize, usize, usize)> {
    let mut offset = 0;
    let mut res = Vec::new();
    for (row, line) in lines.iter().rev().enumerate().skip(1) {
        for (col, ch) in line.chars().enumerate() {
            match ch {
                'A' => offset = col,
                'T' => res.push((col - offset, row - 1, 1)),

                'H' => res.push((col - offset, row - 1, 2)),

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

fn score_target(catapult: usize, target: &(usize, usize, usize)) -> Option<usize> {
    let (x, y, hp) = target;
    if (x + y - catapult) % 3 == 0 {
        let power = (x + y - catapult) / 3;
        Some(hp * (catapult + 1) * power)
    } else {
        None
    }
    // for height in 1..4 {
    //     let h = height as f64;
    //     let (x, y) = (target.0 as f64, target.1 as f64);
    //     let power = (x + y - h - 2.0) / 3.0 + 1.0;
    //     if power.fract() == 0.0 {
    //         return target.2 * height * power as usize;
    //     }
    // }
    // panic!("Not able to hit target {target:?}");
}

/*
.....................
.....................
.......#..........#.. (17 5)
......1..........1...
.....2..........2....
.C..3..........3.....
.B.4..........4......
.A...........5.......
=====================
(17 5)
X - Y = 12
A(P4) hits at H=1 takes 12
A(P5) miss
A(P6) hits at H=4 takes 15
B(P4) miss
B(P5) hits at H=3 takes 19
C(P4) hits at H=2 takes 15
*/

fn moving_target(start: &(usize, usize)) -> usize {
    0
}

fn stationary(targets: Vec<(usize, usize, usize)>) -> usize {
    targets
        .iter()
        .map(|t| (0..3).filter_map(|i| score_target(i, t)).sum::<usize>())
        .sum()
}

fn moving(positions: Vec<(usize, usize)>) -> usize {
    positions.iter().map(moving_target).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![(7, 1, 1), (7, 0, 1), (9, 0, 1)];
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
        let actual = stationary(targets);
        let expected = 13;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_test_moving() {
        let expected = 2;
        let actual = moving_target(&(6, 5));
        assert_eq!(expected, actual);
    }
}

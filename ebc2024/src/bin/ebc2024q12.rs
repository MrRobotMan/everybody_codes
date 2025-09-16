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
    let a = x + y - catapult;
    if a % 3 == 0 {
        let power = a / 3;
        Some(hp * (catapult + 1) * power)
    } else {
        None
    }
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

fn moving_target(catapult: usize, start: &(usize, usize)) -> Option<usize> {
    let &(mut x0, mut y0) = start;
    // Wait 1 time unit if X is odd. Otherwise they won't hit in discrete time.
    if x0 & 1 == 1 {
        x0 -= 1;
        y0 -= 1;
    };

    // Max height is at a distance of half of x.
    let x1 = x0 / 2;
    let y1 = y0 - x1;
    let h = catapult + 1;

    // In line with a catapult. Hit on the way up.
    if x0 + catapult == y0 {
        return Some(h * x1);
    }

    // Hit in the horizontal section
    if let Some(power) = y1.checked_sub(catapult)
        && (power..=2 * power).contains(&x1)
    {
        return Some(h * power);
    }

    // Hit on the way back down.
    if y1 <= x1 + catapult {
        score_target(catapult, &(x1, y1, 1))
    } else {
        None
    }
}

fn stationary(targets: Vec<(usize, usize, usize)>) -> usize {
    targets
        .iter()
        .map(|t| (0..3).filter_map(|i| score_target(i, t)).sum::<usize>())
        .sum()
}

fn moving(positions: Vec<(usize, usize)>) -> usize {
    positions
        .iter()
        .map(|t| {
            (0..3)
                .filter_map(|c| moving_target(c, t))
                .min()
                .unwrap_or(0)
        })
        .sum()
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
        let expected = Some(2);
        let actual = (0..3).filter_map(|c| moving_target(c, &(6, 5))).min();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_test_moving_total() {
        let expected = 13;
        let actual = moving(vec![(6, 5), (6, 7), (10, 5), (5, 5)]);
        assert_eq!(expected, actual);
    }
}

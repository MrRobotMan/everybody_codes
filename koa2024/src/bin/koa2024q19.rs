use std::collections::{HashMap, HashSet};

use puzlib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest19.1.txt");
    let instructions = parse_instructions(&input[0]);
    let message = input[1..]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 1: {}", part_one(instructions, message));

    let input = read_lines("ebc2024/inputs/quest19.2.txt");
    let instructions = parse_instructions(&input[0]);
    let message = input[1..]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 2: {}", part_two(instructions, message));

    let input = read_lines("ebc2024/inputs/quest19.3.txt");
    let instructions = parse_instructions(&input[0]);
    let message = input[1..]
        .iter()
        .map(|line| line.chars().collect())
        .collect();
    println!("Part 3: {}", part_three(instructions, message));
}

fn part_one(instruction: Vec<Rotation>, mut message: Vec<Vec<char>>) -> String {
    rotate(&instruction, &mut message);
    read_message(&message)
}

fn part_two(instruction: Vec<Rotation>, mut message: Vec<Vec<char>>) -> String {
    for _ in 0..100 {
        rotate(&instruction, &mut message);
    }
    read_message(&message)
}

fn part_three(instruction: Vec<Rotation>, message: Vec<Vec<char>>) -> String {
    let mut step = (0..message.len())
        .map(|row| {
            (0..message[0].len())
                .map(|col| (row, col))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    rotate(&instruction, &mut step);
    // Map final postion to where it started.
    let translation = (0..message.len())
        .flat_map(|row| {
            (0..message[0].len()).map({
                let cl = step.clone();
                move |col| (cl[row][col], (row, col))
            })
        })
        .collect::<HashMap<_, _>>();
    let mut cycles = vec![];
    let mut visted = HashSet::new();
    for row in 0..message.len() {
        for col in 0..message[0].len() {
            if visted.contains(&(row, col)) {
                continue;
            }
            let mut cycle = vec![];
            let (mut r, mut c) = (row, col);
            loop {
                if !visted.insert((r, c)) {
                    break;
                };
                cycle.push((r, c));
                (r, c) = translation[&(r, c)];
            }
            cycles.push(cycle);
        }
    }
    let mut new_message = message.clone();
    for cycle in cycles {
        for (idx, (row, col)) in cycle.iter().enumerate() {
            let (r, c) = cycle[(idx + 1048576000) % cycle.len()];
            new_message[r][c] = message[*row][*col];
        }
    }
    read_message(&new_message)
}

const ROT_CW: [(i64, i64); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

const ROT_CCW: [(i64, i64); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
];

fn rotate<T: Copy>(instruction: &[Rotation], message: &mut [Vec<T>]) {
    let mut instructions = instruction.iter().cycle();
    for row in 1..message.len() - 1 {
        for col in 1..message[0].len() - 1 {
            let (row, col) = (row as i64, col as i64);
            let matrix = match instructions.next().unwrap() {
                Rotation::Clockwise => ROT_CW,
                Rotation::CounterClockwise => ROT_CCW,
            };
            let start = message[(row + matrix[0].0) as usize][(col + matrix[0].1) as usize];
            for delta in 1..8 {
                message[(row + matrix[delta - 1].0) as usize]
                    [(col + matrix[delta - 1].1) as usize] =
                    message[(row + matrix[delta].0) as usize][(col + matrix[delta].1) as usize];
            }
            message[(row + matrix[7].0) as usize][(col + matrix[7].1) as usize] = start;
        }
    }
}

fn read_message(message: &[Vec<char>]) -> String {
    let mut res = String::new();
    let mut start = false;
    for row in message {
        for ch in row {
            if ch == &'<' {
                return res;
            }
            if start {
                res.push(*ch);
            }
            if ch == &'>' {
                start = true
            }
        }
    }
    res
}

fn parse_instructions(instruction: &str) -> Vec<Rotation> {
    instruction
        .chars()
        .filter_map(|ch| match ch {
            'R' => Some(Rotation::Clockwise),
            'L' => Some(Rotation::CounterClockwise),
            _ => None,
        })
        .collect()
}

#[derive(Debug, Copy, Clone)]
enum Rotation {
    Clockwise,
    CounterClockwise,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_one() {
        let expected = "WIN";
        let mut message = ">-IN-
-----
W---<"
            .lines()
            .map(|l| l.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        rotate(
            &[Rotation::CounterClockwise, Rotation::Clockwise],
            &mut message,
        );
        let actual = read_message(&message);
        assert_eq!(expected, actual);
    }
}

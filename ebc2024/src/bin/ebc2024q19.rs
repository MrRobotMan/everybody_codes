use std::collections::HashMap;

use puzlib::{Dir, Vec2D, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest19.1.txt");
    let instructions = parse_instructions(&input[0]);
    let message = read_message(&input[1..]);
    let rows = input[1..].len();
    let cols = input[1].len();
    println!("Part 1: {}", part_one(instructions, message, rows, cols));

    let _input = read_lines("ebc2024/inputs/quest19.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest19.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(
    instruction: Vec<Rotation>,
    mut message: HashMap<Vec2D<usize>, char>,
    rows: usize,
    cols: usize,
) -> String {
    let mut rotations = instruction.iter().cycle();
    for row in 1..rows - 1 {
        for col in 1..cols - 1 {
            let rotation = rotations.next().unwrap();
            let rotation_point = Vec2D(row, col);
            let mut points = Dir::compass_unchecked(&rotation_point);
            if matches!(rotation, Rotation::CounterClockwise) {
                points = points
                    .into_iter()
                    .rev()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap();
            }
            rotate(points, &mut message);
        }
    }
    let start = message.iter().find(|(_, c)| **c == '>').unwrap().0;
    let end = message.iter().find(|(_, c)| **c == '<').unwrap().0;
    assert_eq!(start.0, end.0);
    (start.1 + 1..end.1)
        .map(|col| message[&Vec2D(start.0, col)])
        .collect()
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn rotate(points: [Vec2D<usize>; 8], message: &mut HashMap<Vec2D<usize>, char>) {
    let mut cur = message[&points[7]];
    for p in points {
        let next = message[&p];
        message.entry(p).and_modify(|e| *e = cur);
        cur = next;
    }
}

fn read_message(text: &[String]) -> HashMap<Vec2D<usize>, char> {
    text.iter()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, ch)| (Vec2D(row, col), ch))
        })
        .collect()
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
    fn test_rotate_cw() {
        let expected = HashMap::from([
            (Vec2D(0, 0), 'H'),
            (Vec2D(0, 1), 'A'),
            (Vec2D(0, 2), 'B'),
            (Vec2D(1, 2), 'C'),
            (Vec2D(2, 2), 'D'),
            (Vec2D(2, 1), 'E'),
            (Vec2D(2, 0), 'F'),
            (Vec2D(1, 0), 'G'),
        ]);
        let mut actual = HashMap::from([
            (Vec2D(0, 0), 'A'),
            (Vec2D(0, 1), 'B'),
            (Vec2D(0, 2), 'C'),
            (Vec2D(1, 2), 'D'),
            (Vec2D(2, 2), 'E'),
            (Vec2D(2, 1), 'F'),
            (Vec2D(2, 0), 'G'),
            (Vec2D(1, 0), 'H'),
        ]);
        show(&expected, 3, 3);
        println!();
        show(&actual, 3, 3);
        rotate(Dir::compass_unchecked(&Vec2D(1, 1)), &mut actual);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_rotate_ccw() {
        let expected = HashMap::from([
            (Vec2D(0, 0), 'B'),
            (Vec2D(0, 1), 'C'),
            (Vec2D(0, 2), 'D'),
            (Vec2D(1, 2), 'E'),
            (Vec2D(2, 2), 'F'),
            (Vec2D(2, 1), 'G'),
            (Vec2D(2, 0), 'H'),
            (Vec2D(1, 0), 'A'),
        ]);
        let mut actual = HashMap::from([
            (Vec2D(0, 0), 'A'),
            (Vec2D(0, 1), 'B'),
            (Vec2D(0, 2), 'C'),
            (Vec2D(1, 2), 'D'),
            (Vec2D(2, 2), 'E'),
            (Vec2D(2, 1), 'F'),
            (Vec2D(2, 0), 'G'),
            (Vec2D(1, 0), 'H'),
        ]);
        rotate(
            Dir::compass_unchecked(&Vec2D(1, 1))
                .into_iter()
                .rev()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            &mut actual,
        );
        show(&expected, 3, 3);
        println!();
        show(&actual, 3, 3);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_example_one() {
        let expected = "WIN";
        let actual = part_one(
            vec![Rotation::CounterClockwise, Rotation::Clockwise],
            read_message(
                &">-IN-
-----
W---<"
                    .lines()
                    .map(|l| l.into())
                    .collect::<Vec<_>>(),
            ),
            3,
            5,
        );
        assert_eq!(expected, actual);
    }

    fn show(stuff: &HashMap<Vec2D<usize>, char>, rows: usize, cols: usize) {
        for row in 0..rows {
            for col in 0..cols {
                print!("{}", stuff.get(&Vec2D(row, col)).unwrap_or(&'x'));
            }
            println!();
        }
    }
}

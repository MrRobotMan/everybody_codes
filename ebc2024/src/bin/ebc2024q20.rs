use std::collections::HashMap;

use puzlib::{Dir, Vec2D, read_grid_to_map};

fn main() {
    let input = read_grid_to_map("ebc2024/inputs/quest20.1.txt");
    println!("Part 1: {}", part_one(input));

    let _input = read_grid_to_map("ebc2024/inputs/quest20.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_grid_to_map("ebc2024/inputs/quest20.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(input: Vec<((usize, usize), char)>) -> i64 {
    let mut start = Vec2D(0, 0);
    let mut map: Map = HashMap::new();
    for (node, ch) in input {
        if ch == 'S' {
            start = Vec2D(node.0 as i64, node.1 as i64);
        }
        map.insert(Vec2D(node.0 as i64, node.1 as i64), ch.into());
    }
    let mut step: HashMap<Glider, i64> = HashMap::from_iter(
        Dir::cardinals_unchecked(&Vec2D(0, 0))
            .iter()
            .map(|heading| (Glider::new(start, *heading), 1000)),
    );

    let mut time = 0;
    while time < 100 {
        let mut next_step = step.clone();
        for (glider, altitude) in step {
            for (state, mut new_altitude) in glider.moves(altitude, &map) {
                let cur_alt = next_step.entry(state).or_insert(new_altitude);
                *cur_alt = *cur_alt.max(&mut new_altitude);
            }
        }
        step = next_step;
        time += 1;
    }
    *step.values().max().unwrap()
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

type Map = HashMap<Vec2D<i64>, Segment>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Glider {
    pos: Vec2D<i64>,
    heading: Vec2D<i64>,
}

impl Glider {
    fn new(pos: Vec2D<i64>, heading: Vec2D<i64>) -> Self {
        Self { pos, heading }
    }

    fn moves(&self, altitude: i64, map: &Map) -> Vec<(Self, i64)> {
        let mut res = vec![];
        let directions = if self.heading.0 == 0 {
            [Vec2D(-1, 0), Vec2D(1, 0), self.heading]
        } else {
            [Vec2D(0, -1), Vec2D(0, 1), self.heading]
        };
        for dir in directions {
            let next_loc = self.pos + dir;
            if let Some(segment) = map.get(&next_loc)
                && let Some(delta) = segment.delta_altitude()
            {
                res.push((
                    Self {
                        pos: next_loc,
                        heading: dir,
                    },
                    delta + altitude,
                ))
            }
        }
        res
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Segment {
    None,
    Warm,
    Cold,
    Obstacle,
}
impl Segment {
    fn delta_altitude(&self) -> Option<i64> {
        match self {
            Segment::None => Some(-1),
            Segment::Warm => Some(1),
            Segment::Cold => Some(-2),
            Segment::Obstacle => None,
        }
    }
}

impl From<char> for Segment {
    fn from(value: char) -> Self {
        match value {
            '.' | 'S' => Self::None,
            '+' => Self::Warm,
            '-' => Self::Cold,
            '#' => Self::Obstacle,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 1045;
        let input = read_grid_to_map(
            "#....S....#
#.........#
#---------#
#.........#
#..+.+.+..#
#.+-.+.++.#
#.........#",
        );
        let actual = part_one(input);
        assert_eq!(expected, actual);
    }
}

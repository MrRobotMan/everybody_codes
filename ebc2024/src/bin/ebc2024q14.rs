use std::{
    collections::{HashMap, HashSet, VecDeque},
    str::FromStr,
};

use puzlib::{Vec3D, get_path, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest14.1.txt");
    println!("Part 1: {}", part_one(parse_input(input).pop().unwrap()));

    let input = read_lines("ebc2024/inputs/quest14.2.txt");
    println!("Part 2: {}", part_two(parse_input(input)));

    let input = read_lines("ebc2024/inputs/quest14.3.txt");
    println!("Part 3: {}", part_three(parse_input(input)));
}

fn parse_input(input: Vec<String>) -> Vec<Vec<Instruction>> {
    input
        .iter()
        .map(|l| {
            l.split(',')
                .map(|i| i.parse().unwrap())
                .collect::<Vec<Instruction>>()
        })
        .collect()
}

fn part_one(instructions: Vec<Instruction>) -> i64 {
    let mut max_height = 0;
    let mut cur = Vec3D(0, 0, 0);
    for inst in instructions {
        cur += inst.as_vec3d();
        max_height = max_height.max(cur.2);
    }
    max_height
}

fn part_two(instructions: Vec<Vec<Instruction>>) -> usize {
    let mut visited = HashSet::new();
    for branch in instructions {
        let mut cur = Vec3D(0, 0, 0);
        for inst in branch {
            for _ in 0..inst.scale {
                cur += inst.direction;
                visited.insert(cur);
            }
        }
    }
    visited.len()
}

fn part_three(instructions: Vec<Vec<Instruction>>) -> i64 {
    let mut leaves = vec![];
    let mut visited = HashSet::new();
    let mut height = 0;
    for branch in instructions {
        let mut cur = Vec3D(0, 0, 0);
        for inst in branch {
            for _ in 0..inst.scale {
                cur += inst.direction;
                height = height.max(cur.2);
                visited.insert(cur);
            }
        }
        leaves.push(cur);
    }
    let mut best = i64::MAX;
    for z in 1..=height {
        if let Some(found) = leaves
            .iter()
            .map(|l| bfs(Vec3D(0, 0, z), l, &visited))
            .sum()
        {
            best = best.min(found)
        }
    }
    best
}

fn bfs(start: Vec3D<i64>, end: &Vec3D<i64>, graph: &HashSet<Vec3D<i64>>) -> Option<i64> {
    let mut path = HashMap::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(start);
    while let Some(node) = to_visit.pop_front() {
        if node == *end {
            let path = get_path(path, node, &start);
            return Some(path.len() as i64 - 1);
        }
        for next_move in moves(&node, graph) {
            if path.contains_key(&next_move) {
                continue;
            }
            to_visit.push_back(next_move);
            path.insert(next_move, node);
        }
    }
    None
}

fn moves(node: &Vec3D<i64>, graph: &HashSet<Vec3D<i64>>) -> Vec<Vec3D<i64>> {
    [
        Vec3D(0, 0, 1),
        Vec3D(0, 0, -1),
        Vec3D(0, 1, 0),
        Vec3D(0, -1, 0),
        Vec3D(1, 0, 0),
        Vec3D(-1, 0, 0),
    ]
    .iter()
    .filter_map(|n| {
        let next = node + n;
        if graph.contains(&next) {
            Some(next)
        } else {
            None
        }
    })
    .collect()
}

#[derive(Debug, Default)]
struct Instruction {
    direction: Vec3D<i64>,
    scale: i64,
}

impl Instruction {
    fn as_vec3d(&self) -> Vec3D<i64> {
        self.direction.scale(self.scale)
    }
}

impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let direction = match s[..1].as_ref() {
            "U" => Vec3D(0, 0, 1),
            "D" => Vec3D(0, 0, -1),
            "L" => Vec3D(0, 1, 0),
            "R" => Vec3D(0, -1, 0),
            "F" => Vec3D(1, 0, 0),
            "B" => Vec3D(-1, 0, 0),
            c => return Err(format!("Unknown direction {c}")),
        };
        let scale = s[1..].parse().unwrap();
        Ok(Self { direction, scale })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 7;
        let instructions = "U5,R3,D2,L5,U4,R5,D2"
            .split(',')
            .map(|s| s.parse().unwrap())
            .collect::<Vec<Instruction>>();
        let actual = part_one(instructions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two() {
        let expected = 32;
        let instructions = "U5,R3,D2,L5,U4,R5,D2\nU6,L1,D2,R3,U2,L1"
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<Instruction>>()
            })
            .collect::<Vec<_>>();
        println!("{instructions:?}");
        let actual = part_two(instructions);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let expected = 46;
        let instructions = "U20,L1,B1,L2,B1,R2,L1,F1,U1\nU10,F1,B1,R1,L1,B1,L1,F1,R2,U1\nU30,L2,F1,R1,B1,R1,F2,U1,F1\nU25,R1,L2,B1,U1,R2,F1,L2\nU16,L1,B1,L1,B3,L1,B1,F1"
            .lines()
            .map(|l| {
                l.split(',')
                    .map(|s| s.parse().unwrap())
                    .collect::<Vec<Instruction>>()
            })
            .collect::<Vec<_>>();
        let actual = part_three(instructions);
        assert_eq!(expected, actual);
    }
}

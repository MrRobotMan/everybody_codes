use std::collections::{HashMap, HashSet};

use puzlib::{Dir, Graph, Vec2D, bfs, read_grid_to_map};

fn main() {
    let input = read_grid_to_map("ebc2024/inputs/quest15.1.txt");
    let mut garden: Garden = input.into();
    println!("Part 1: {}", garden.find_path_to_herbs());

    let _input = read_grid_to_map("ebc2024/inputs/quest15.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_grid_to_map("ebc2024/inputs/quest15.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

#[derive(Debug, Default)]
struct Garden {
    map: HashMap<Vec2D<i64>, char>,
    start: Vec2D<i64>,
    dims: (usize, usize),
    to_find: HashSet<char>,
}

impl Garden {
    fn find_path_to_herbs(&mut self) -> usize {
        let mut dist = 0;
        let mut cur = self.start;
        while !self.to_find.is_empty() {
            let mut path = bfs(&cur, self).unwrap();
            cur = path.pop().unwrap();
            self.to_find.remove(&self.map[&cur]);
            dist += path.len();
        }
        self.to_find.insert('S');
        dist + bfs(&cur, self).unwrap().len() - 1
    }
}

impl Graph for Garden {
    type Node = Vec2D<i64>;

    fn height(&self) -> usize {
        self.dims.0
    }

    fn width(&self) -> usize {
        self.dims.1
    }

    fn moves(&self, node: &Self::Node) -> Vec<Self::Node> {
        Dir::<i64>::cardinals()
            .iter()
            .filter_map(|d| {
                let next = node + d;
                if self.map.contains_key(&next) {
                    Some(next)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_done(&self, node: &Self::Node) -> bool {
        self.to_find.contains(&self.map[node])
    }
}

impl From<Vec<((usize, usize), char)>> for Garden {
    fn from(value: Vec<((usize, usize), char)>) -> Self {
        let mut garden = Self::default();
        for ((x, y), char) in value {
            garden.dims.0 = garden.dims.0.max(x);
            garden.dims.0 = garden.dims.1.max(y);
            let x = x as i64;
            let y = y as i64;
            match char {
                '.' => {
                    if x == 0 {
                        garden.map.insert(Vec2D(x, y), 'S');
                        garden.start = Vec2D(x, y);
                    } else {
                        garden.map.insert(Vec2D(x, y), '.');
                    }
                }
                '#' => (),
                c => {
                    garden.map.insert(Vec2D(x, y), c);
                    garden.to_find.insert(c);
                }
            }
        }
        garden
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 26;
        let mut garden: Garden = read_grid_to_map(
            "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########",
        )
        .into();
        println!("{garden:?}");
        let actual = garden.find_path_to_herbs();
        assert_eq!(expected, actual);
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

use puzlib::{Dir, Vec2D, read_grid_to_map};

fn main() {
    let input = read_grid_to_map("ebc2024/inputs/quest15.1.txt");
    let garden: Garden = input.into();
    println!("Part 1: {}", garden.find_path_to_herbs());

    let input = read_grid_to_map("ebc2024/inputs/quest15.2.txt");
    let garden: Garden = input.into();
    println!("Part 2: {}", garden.find_path_to_herbs());

    let input = read_grid_to_map("ebc2024/inputs/quest15.3.txt");
    let garden: Garden = input.into();
    println!("Part 3: {}", garden.find_path_to_herbs());
}

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
struct State {
    node: Vec2D<i64>,
    collected: u32,
    steps: usize,
}

#[derive(Debug, Default)]
struct Garden {
    map: HashMap<Vec2D<i64>, u32>,
    start: Vec2D<i64>,
    herb_mask: u32,
}

impl Garden {
    fn find_path_to_herbs(&self) -> usize {
        let mut visited = HashSet::new();
        let mut most_herbs = 0;
        let mut to_visit = VecDeque::new();
        to_visit.push_back(State {
            node: self.start,
            collected: self.map[&self.start],
            steps: 0,
        });
        while let Some(State {
            node,
            collected,
            steps,
        }) = to_visit.pop_front()
        {
            if node == self.start && collected == self.herb_mask {
                return steps;
            }
            let coll = if self.map[&node] != self.map[&self.start] {
                collected | self.map[&node]
            } else {
                collected
            };
            let herbs = u32::count_ones(coll);
            // Prune paths that have collected 2 fewer herbs than the best path.
            if herbs + 2 < most_herbs {
                continue;
            }
            most_herbs = most_herbs.max(herbs);
            for next in self.neighbors(&node) {
                let mut next_state = State {
                    node: next,
                    collected: coll,
                    steps: 0,
                };
                if visited.insert(next_state) {
                    next_state.steps = steps + 1;
                    to_visit.push_back(next_state);
                }
            }
        }
        0
    }

    fn neighbors(&self, node: &Vec2D<i64>) -> Vec<Vec2D<i64>> {
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
}

impl From<Vec<((usize, usize), char)>> for Garden {
    fn from(value: Vec<((usize, usize), char)>) -> Self {
        let mut garden = Self::default();
        for ((x, y), char) in value {
            let x = x as i64;
            let y = y as i64;
            match char {
                '.' => {
                    if x == 0 {
                        let v = 1 << (b'S' - b'A');
                        garden.map.insert(Vec2D(x, y), v);
                        garden.herb_mask |= v;
                        garden.start = Vec2D(x, y);
                    } else {
                        garden.map.insert(Vec2D(x, y), 0);
                    }
                }
                '#' | '~' => (),
                c => {
                    let v = 1 << (c as u8 - b'A');
                    garden.map.insert(Vec2D(x, y), v);
                    garden.herb_mask |= v;
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
        let garden: Garden = read_grid_to_map(
            "#####.#####
#.........#
#.######.##
#.........#
###.#.#####
#H.......H#
###########",
        )
        .into();
        let actual = garden.find_path_to_herbs();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two() {
        let expected = 38;
        let garden: Garden = read_grid_to_map(
            "##########.##########
#...................#
#.###.##.###.##.#.#.#
#..A#.#..~~~....#A#.#
#.#...#.~~~~~...#.#.#
#.#.#.#.~~~~~.#.#.#.#
#...#.#.B~~~B.#.#...#
#...#....BBB..#....##
#C............#....C#
#####################",
        )
        .into();
        let actual = garden.find_path_to_herbs();
        assert_eq!(expected, actual);
    }
}

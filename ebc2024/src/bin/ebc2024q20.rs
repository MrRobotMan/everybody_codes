use std::collections::HashMap;

use puzlib::{Dir, Vec2D, read_grid_to_map};

fn main() {
    let input = read_grid_to_map("ebc2024/inputs/quest20.1.txt");
    println!("Part 1: {}", part_one(input));

    let input = read_grid_to_map("ebc2024/inputs/quest20.2.txt");
    println!("Part 2: {}", part_two(input));

    let input = read_grid_to_map("ebc2024/inputs/quest20.3.txt");
    println!("Part 3: {}", part_three(input, 384400));
}

fn part_one(input: Vec<((usize, usize), char)>) -> i64 {
    let map = Map::new(input);
    let mut step: HashMap<Glider, i64> = HashMap::from_iter(
        Dir::cardinals_unchecked(&Vec2D(0, 0))
            .iter()
            .map(|heading| (Glider::new(map.start, *heading), 1_000)),
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

fn part_two(input: Vec<((usize, usize), char)>) -> usize {
    let map = Map::new(input);
    let mut step: HashMap<Glider, i64> = HashMap::from_iter(
        Dir::cardinals_unchecked(&Vec2D(0, 0))
            .iter()
            .map(|heading| (Glider::new(map.start, *heading), 10_000)),
    );
    let mut time = 0;
    while !step.is_empty() {
        let mut next_step = step.clone();
        for (glider, altitude) in step {
            if altitude >= 10_000 && glider.returned(&map) {
                return time;
            }
            for (state, mut new_altitude) in glider.moves(altitude, &map) {
                let cur_alt = next_step.entry(state).or_insert(new_altitude);
                *cur_alt = *cur_alt.max(&mut new_altitude);
            }
        }
        step = next_step;
        time += 1;
    }
    usize::MAX
}

fn part_three(input: Vec<((usize, usize), char)>, altitude: i64) -> i64 {
    let map = Map::new(input);
    let mut furthest_reach: Option<i64> = None;
    let mut best_col = None;
    let mut min_drop = i64::MIN;
    let Vec2D(rows, cols) = *map.grid.keys().max().unwrap() + Vec2D(1, 1);
    let mut best_offset = cols as i64;
    'outer: for col in 0..cols {
        let mut alt = 0;
        for row in 0..rows {
            if let Some((_, delta)) = map.get(&Vec2D(row, col)) {
                alt += delta;
            } else {
                continue 'outer;
            }
        }
        let lateral_offset = col.abs_diff(map.start.1) as i64;
        if alt > min_drop || (alt == min_drop && lateral_offset < best_offset) {
            min_drop = alt;
            best_offset = lateral_offset;
            best_col = Some(col);
        }
    }

    let Some(best_col) = best_col else {
        panic!("Can't fly down a column")
    };

    let mut step = HashMap::<Glider, i64>::from([
        (Glider::new(map.start, Vec2D(1, 0)), altitude),
        (Glider::new(map.start, Vec2D(0, 1)), altitude),
        (Glider::new(map.start, Vec2D(0, -1)), altitude),
    ]);

    // Move to the ideal column. Will be at the bottom of the known grid.
    while !step.is_empty() {
        let mut next_step = HashMap::new();
        for (glider, alt) in step.iter() {
            if alt == &0 {
                furthest_reach = match furthest_reach {
                    Some(reach) => Some(reach.max(glider.pos.0)),
                    None => Some(glider.pos.0),
                };
            }
            for (state, new_altitude) in glider.moves(*alt, &map) {
                if new_altitude < 0 {
                    continue;
                }
                if state.pos.1 <= glider.pos.1 && glider.pos.1 < best_col {
                    continue;
                }
                if state.pos.1 >= glider.pos.1 && glider.pos.1 > best_col {
                    continue;
                }
                if let Some(cur_alt) = step.get(&state) {
                    next_step.insert(state, *cur_alt.max(&new_altitude));
                } else {
                    next_step.insert(state, new_altitude);
                }
            }
        }
        step = next_step;
    }

    // If we hit 0 on the first pass, return that.
    if let Some(dist) = furthest_reach {
        return dist;
    }
    let altitude = altitude - best_offset - 1;
    let grids_traversed = altitude / min_drop.abs();
    let mut distance = grids_traversed * rows;
    let mut cur_altitude = altitude - (grids_traversed * min_drop.abs());
    let mut row = 1;
    distance += 1;
    while cur_altitude > 0 {
        if let Some((_, delta)) = map.get(&Vec2D(row, best_col)) {
            cur_altitude += delta;
            distance += 1;
            row += 1;
        }
    }
    distance
}

#[derive(Debug, Clone, Default)]
struct Map {
    grid: HashMap<Vec2D<i64>, Segment>,
    start: Vec2D<i64>,
    checkpoint_a: Option<Vec2D<i64>>,
    checkpoint_b: Option<Vec2D<i64>>,
    checkpoint_c: Option<Vec2D<i64>>,
}

impl Map {
    fn new(input: Vec<((usize, usize), char)>) -> Self {
        let mut map = Self::default();
        for (node, ch) in input {
            if ch == 'S' {
                map.start = Vec2D(node.0 as i64, node.1 as i64);
            }
            if ch == 'A' {
                map.checkpoint_a = Some(Vec2D(node.0 as i64, node.1 as i64));
            }
            if ch == 'B' {
                map.checkpoint_b = Some(Vec2D(node.0 as i64, node.1 as i64));
            }
            if ch == 'C' {
                map.checkpoint_c = Some(Vec2D(node.0 as i64, node.1 as i64));
            }
            map.grid
                .insert(Vec2D(node.0 as i64, node.1 as i64), ch.into());
        }
        map
    }

    fn get(&self, pos: &Vec2D<i64>) -> Option<(Segment, i64)> {
        if let Some(segment) = self.grid.get(pos)
            && let Some(delta) = segment.delta_altitude()
        {
            Some((*segment, delta))
        } else {
            None
        }
    }

    fn at_checkpoint(&self, checkpoint: Checkpoint, location: Vec2D<i64>) -> bool {
        let check_loc = match checkpoint {
            Checkpoint::A => self.checkpoint_a,
            Checkpoint::B => self.checkpoint_b,
            Checkpoint::C => self.checkpoint_c,
            Checkpoint::Start => Some(self.start),
        };
        if let Some(check) = check_loc
            && check == location
        {
            true
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Checkpoint {
    A,
    B,
    C,
    Start,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Glider {
    pos: Vec2D<i64>,
    heading: Vec2D<i64>,
    checkpoints: u8,
}

impl Glider {
    fn new(pos: Vec2D<i64>, heading: Vec2D<i64>) -> Self {
        Self {
            pos,
            heading,
            checkpoints: 0,
        }
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
            let mut checkpoints = self.checkpoints;
            if (map.at_checkpoint(Checkpoint::A, next_loc) && checkpoints != 0)
                || (map.at_checkpoint(Checkpoint::B, next_loc) && checkpoints != 1)
                || (map.at_checkpoint(Checkpoint::C, next_loc) && checkpoints != 2)
                || (map.at_checkpoint(Checkpoint::Start, next_loc) && checkpoints != 3)
            {
                continue;
            }
            checkpoints += (map.at_checkpoint(Checkpoint::A, next_loc) as u8)
                + (map.at_checkpoint(Checkpoint::B, next_loc) as u8)
                + (map.at_checkpoint(Checkpoint::C, next_loc) as u8);

            if let Some((_, delta)) = map.get(&next_loc) {
                res.push((
                    Self {
                        pos: next_loc,
                        heading: dir,
                        checkpoints,
                    },
                    delta + altitude,
                ))
            }
        }
        res
    }

    fn returned(&self, map: &Map) -> bool {
        self.pos == map.start && self.checkpoints == 3
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
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
            '.' | 'S' | 'A' | 'B' | 'C' => Self::None,
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

    #[test]
    fn test_two() {
        let expected = 24;
        let input = read_grid_to_map(
            "####S####
#-.+++.-#
#.+.+.+.#
#-.+.+.-#
#A+.-.+C#
#.+-.-+.#
#.+.B.+.#
#########",
        );
        let actual = part_two(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let expected = 768790;
        let input = read_grid_to_map(
            "#......S......#
#-...+...-...+#
#.............#
#..+...-...+..#
#.............#
#-...-...+...-#
#.............#
#..#...+...+..#",
        );
        let actual = part_three(input, 384400);
        assert_eq!(expected, actual);
    }
}

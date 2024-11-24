use std::collections::{HashMap, HashSet};

use ebclib::read_grid;

fn main() {
    let input = read_grid("ebc2024/inputs/quest03.1.txt");
    println!("Part 1: {}", part_one(input));

    let input = read_grid("ebc2024/inputs/quest03.2.txt");
    println!("Part 2: {}", part_one(input));

    let input = read_grid("ebc2024/inputs/quest03.3.txt");
    println!("Part 3: {}", part_three(input));
}

const DIRS: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
const ALL_DIRS: [(i64, i64); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];

fn part_one(grid: Vec<Vec<char>>) -> usize {
    let ((rows, cols), mut map) = get_grid(grid);
    let mut next_step = 1;
    while step(rows, cols, &mut map, next_step) {
        next_step += 1;
    }
    map.values().sum()
}

fn part_three(grid: Vec<Vec<char>>) -> usize {
    let ((rows, cols), mut map) = get_grid(grid);
    let mut next_step = 1;
    #[cfg(test)]
    show_grid(rows, cols, &map);
    while step_all_ways(rows, cols, &mut map, next_step) {
        #[cfg(test)]
        show_grid(rows, cols, &map);
        next_step += 1;
    }
    map.values().sum()
}

fn step(rows: i64, cols: i64, grid: &mut HashMap<(i64, i64), usize>, step: usize) -> bool {
    let mut to_update = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            if DIRS
                .iter()
                .all(|d| match grid.get(&(row + d.0, col + d.1)) {
                    Some(v) => *v == step,
                    None => true,
                })
            {
                to_update.insert((row, col));
            }
        }
    }
    for loc in to_update.iter() {
        grid.entry(*loc).and_modify(|v| *v += 1);
    }
    !to_update.is_empty()
}

fn step_all_ways(rows: i64, cols: i64, grid: &mut HashMap<(i64, i64), usize>, step: usize) -> bool {
    let mut to_update = HashSet::new();
    for row in 0..rows {
        for col in 0..cols {
            if grid[&(row, col)] == 0 {
                continue;
            }
            if ALL_DIRS
                .iter()
                .all(|d| match grid.get(&(row + d.0, col + d.1)) {
                    Some(v) => *v == step,
                    None => false,
                })
            {
                to_update.insert((row, col));
            }
        }
    }
    for loc in to_update.iter() {
        grid.entry(*loc).and_modify(|v| *v += 1);
    }
    !to_update.is_empty()
}

fn get_grid(grid: Vec<Vec<char>>) -> ((i64, i64), HashMap<(i64, i64), usize>) {
    let rows = grid.len();
    let cols = grid[0].len();
    let map = grid
        .iter()
        .enumerate()
        .flat_map(|(r, l)| {
            l.iter()
                .enumerate()
                .map(|(c, ch)| {
                    (
                        (r as i64, c as i64),
                        match *ch {
                            '#' => 1,
                            '.' => 0,
                            _ => panic!("Unknown char {ch}"),
                        },
                    )
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i64, i64), usize>>();
    ((rows as i64, cols as i64), map)
}

#[cfg(test)]
fn show_grid(rows: i64, cols: i64, map: &HashMap<(i64, i64), usize>) {
    for row in 0..rows {
        for col in 0..cols {
            print!("{}", map[&(row, col)]);
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let grid = read_grid(
            "..........
..###.##..
...####...
..######..
..######..
...####...
..........",
        );
        assert_eq!(35, part_one(grid));
    }

    #[test]
    fn test_three() {
        let grid = read_grid(
            "..........
..###.##..
...####...
..######..
..######..
...####...
..........",
        );
        assert_eq!(29, part_three(grid));
    }
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use puzlib::{Vec2D, read_grid_to_map};

fn main() {
    let input = read_grid_to_map("ebc2024/inputs/quest17.1.txt")
        .iter()
        .filter_map(|(p, ch)| if *ch == '*' { Some(p.into()) } else { None })
        .collect();
    println!("Part 1: {}", part_one(input));

    let _input = read_grid_to_map("ebc2024/inputs/quest17.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_grid_to_map("ebc2024/inputs/quest17.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(stars: HashSet<Vec2D<usize>>) -> usize {
    // Prim's Mimimum Spanning Tree
    let mut total_distance = 0;
    let mut visited = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    let start = *stars.iter().min().unwrap();
    to_visit.push(Reverse((0, start)));
    while let Some(Reverse((dist, star))) = to_visit.pop() {
        if visited.contains(&star) {
            continue;
        }
        total_distance += dist;
        visited.insert(star);
        for other in stars.iter() {
            if !visited.contains(other) {
                to_visit.push(Reverse((star.manhattan(*other), *other)));
            }
        }
    }

    stars.len() + total_distance
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 16;
        let stars = HashSet::from([
            (1_usize, 5).into(),
            (5_usize, 5).into(),
            (3_usize, 4).into(),
            (1_usize, 1).into(),
            (3_usize, 1).into(),
        ]);
        let actual = part_one(stars);
        assert_eq!(expected, actual);
    }
}

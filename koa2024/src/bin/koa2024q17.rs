use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
};

use puzlib::{Vec2D, read_grid_to_map};

fn main() {
    let input = lines_to_stars(read_grid_to_map("ebc2024/inputs/quest17.1.txt"));
    println!("Part 1: {}", part_one(input));

    let input = lines_to_stars(read_grid_to_map("ebc2024/inputs/quest17.2.txt"));
    println!("Part 2: {}", part_two(input));

    let input = lines_to_stars(read_grid_to_map("ebc2024/inputs/quest17.3.txt"));
    println!("Part 3: {}", part_three(input));
}

fn lines_to_stars(input: Vec<((usize, usize), char)>) -> HashSet<Vec2D<usize>> {
    input
        .iter()
        .filter_map(|(p, ch)| if *ch == '*' { Some(p.into()) } else { None })
        .collect()
}

fn part_one(stars: HashSet<Vec2D<usize>>) -> usize {
    best_constellation(stars)
}

fn part_two(stars: HashSet<Vec2D<usize>>) -> usize {
    best_constellation(stars)
}

fn part_three(mut stars: HashSet<Vec2D<usize>>) -> usize {
    let mut constellations = Vec::new();
    while !stars.is_empty() {
        let cons = find_brilliant_constellation(&stars, 6);
        for star in cons.iter() {
            stars.remove(star);
        }
        constellations.push(best_constellation(cons));
    }
    constellations.sort_by(|a, b| b.cmp(a));
    constellations
        .into_iter()
        .take(3)
        .reduce(|acc, v| acc * v)
        .unwrap()
}

fn best_constellation(stars: HashSet<Vec2D<usize>>) -> usize {
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

fn find_brilliant_constellation(
    stars: &HashSet<Vec2D<usize>>,
    max_dist: usize,
) -> HashSet<Vec2D<usize>> {
    // Prim's Mimimum Spanning Tree
    let mut constellation = HashSet::new();
    let mut to_visit = BinaryHeap::new();
    let start = *stars.iter().min().unwrap();
    to_visit.push(Reverse((0, start)));
    while let Some(Reverse((_, star))) = to_visit.pop() {
        if constellation.contains(&star) {
            continue;
        }
        constellation.insert(star);
        for other in stars.iter() {
            let dist = star.manhattan(*other);
            if !constellation.contains(other) && dist < max_dist {
                to_visit.push(Reverse((dist, *other)));
            }
        }
    }
    constellation
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

    #[test]
    fn test_two() {
        let expected = 15624;
        let actual = part_three(lines_to_stars(read_grid_to_map(
            ".......................................
..*.......*...*.....*...*......**.**...
....*.................*.......*..*..*..
..*.........*.......*...*.....*.....*..
......................*........*...*...
..*.*.....*...*.....*...*........*.....
.......................................",
        )));
        assert_eq!(expected, actual);
    }
}

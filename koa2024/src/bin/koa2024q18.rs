use std::collections::{HashMap, HashSet};

use puzlib::{Dir, Vec2D, read_grid_to_map};

fn main() {
    let input: HashMap<Vec2D<usize>, char> = read_grid_to_map("koa2024/inputs/quest18.1.txt")
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
    println!("Part 1: {}", part_one_two(input));

    let input = read_grid_to_map("koa2024/inputs/quest18.2.txt")
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
    println!("Part 2: {}", part_one_two(input));

    let input = read_grid_to_map("koa2024/inputs/quest18.3.txt")
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
    println!("Part 3: {}", part_three(input));
}

fn part_one_two(nodes: HashMap<Vec2D<usize>, char>) -> usize {
    let starts = get_start(&nodes);
    let palms = nodes
        .iter()
        .filter_map(|(n, c)| if c == &'P' { Some(*n) } else { None })
        .collect::<HashSet<_>>();
    let res = flood_fill(starts, &nodes);
    *res.iter()
        .filter_map(|(n, c)| if palms.contains(n) { Some(c) } else { None })
        .max()
        .unwrap()
}

fn part_three(nodes: HashMap<Vec2D<usize>, char>) -> usize {
    let palms = nodes
        .iter()
        .filter_map(|(n, c)| if c == &'P' { Some(*n) } else { None })
        .collect::<HashSet<_>>();
    let mut best = HashMap::new();
    for node in palms.iter() {
        for (node, time) in flood_fill(vec![*node], &nodes) {
            if palms.contains(&node) {
                continue;
            }
            let ent = best.entry(node).or_default();
            *ent += time;
        }
    }
    *best.values().min().unwrap()
}

fn get_start(nodes: &HashMap<Vec2D<usize>, char>) -> Vec<Vec2D<usize>> {
    let mut starts = vec![];
    let max_col = nodes.keys().map(|n| n.1).max().unwrap();
    let max_row = nodes.keys().map(|n| n.0).max().unwrap();
    for (node, ch) in nodes {
        if (node.0 == 0 || node.0 == max_row || node.1 == 0 || node.1 == max_col) && ch == &'.' {
            starts.push(*node);
        }
    }
    starts
}

fn flood_fill(
    starts: Vec<Vec2D<usize>>,
    input: &HashMap<Vec2D<usize>, char>,
) -> HashMap<Vec2D<usize>, usize> {
    let mut steps = 0;
    let mut visited = HashMap::new();
    let mut nodes = starts;
    loop {
        let mut next = vec![];
        for n in nodes {
            if visited.contains_key(&n) {
                continue;
            }
            visited.insert(n, steps);
            for neighbor in Dir::cardinals(&n) {
                if let Some(neighbor) = neighbor
                    && !visited.contains_key(&neighbor)
                    && let Some(ch) = input.get(&neighbor)
                    && ch != &'#'
                {
                    next.push(neighbor);
                }
            }
        }
        steps += 1;
        if next.is_empty() {
            break;
        }
        nodes = next
    }
    visited
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 11;
        let input = read_grid_to_map(
            "##########
..#......#
#.P.####P#
#.#...P#.#
##########",
        )
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
        let actual = part_one_two(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let expected = 12;
        let input = read_grid_to_map(
            "##########
#.#......#
#.P.####P#
#.#...P#.#
##########",
        )
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
        let actual = part_three(input);
        assert_eq!(expected, actual);
    }
}

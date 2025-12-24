use std::collections::{HashMap, HashSet};

use puzlib::{Dir, Vec2D, read_grid_to_map};

fn main() {
    let input: HashMap<Vec2D<usize>, char> = read_grid_to_map("ebc2024/inputs/quest18.1.txt")
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
    println!("Part 1: {}", flood_fill(input));

    let input = read_grid_to_map("ebc2024/inputs/quest18.2.txt")
        .into_iter()
        .map(|(n, c)| (n.into(), c))
        .collect();
    println!("Part 2: {}", flood_fill(input));

    let _input = read_grid_to_map("ebc2024/inputs/quest18.3.txt");
    println!("Part 3: {}", part_three());
}

fn flood_fill(input: HashMap<Vec2D<usize>, char>) -> usize {
    let start = get_start(&input);
    let mut palms = input
        .iter()
        .filter_map(|(n, c)| if c == &'P' { Some(n) } else { None })
        .collect::<HashSet<_>>();
    let mut steps = 0;
    let mut visited: HashSet<Vec2D<usize>> = HashSet::from_iter(start.iter().copied());
    let mut queue = Vec::from([start]);
    while let Some(nodes) = queue.pop() {
        if palms.is_empty() {
            break;
        }
        steps += 1;
        let mut next = vec![];
        for n in nodes {
            visited.insert(n);
            for neighbor in Dir::cardinals(&n) {
                if let Some(neighbor) = neighbor
                    && !visited.contains(&neighbor)
                    && let Some(ch) = input.get(&neighbor)
                    && ch != &'#'
                {
                    palms.remove(&neighbor);
                    next.push(neighbor);
                }
            }
        }
        if !next.is_empty() {
            queue.push(next);
        }
    }
    steps
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

fn part_three() -> String {
    "Unsolved".into()
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
        let actual = flood_fill(input);
        assert_eq!(expected, actual);
    }
}

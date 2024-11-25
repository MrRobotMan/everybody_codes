use std::collections::HashMap;

use ebclib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest06.1.txt");
    println!("Part 1: {}", part_one(&parse(&input)));

    let _input = read_lines("ebc2024/inputs/quest06.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest06.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(mapping: &HashMap<String, Vec<String>>) -> String {
    let mut paths: HashMap<usize, Vec<String>> = HashMap::new();
    let mut queue = vec![("RR", vec![])];
    while let Some((node, steps)) = queue.pop() {
        if let Some(nodes) = mapping.get(node) {
            for next_node in nodes {
                let mut new_steps = steps.clone();
                new_steps.push(node);
                match next_node.as_ref() {
                    "@" => {
                        new_steps.push("@");
                        paths
                            .entry(steps.len() + 1)
                            .and_modify(|s| s.push(new_steps.iter().copied().collect::<String>()))
                            .or_insert(vec![new_steps.iter().copied().collect::<String>()]);
                    }
                    _ => {
                        queue.push((next_node, new_steps));
                    }
                }
            }
        }
    }
    assert_eq!(paths.values().filter(|p| p.len() == 1).count(), 1);
    paths
        .values()
        .filter_map(|p| {
            if p.len() == 1 {
                Some(p[0].clone())
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn parse<S: AsRef<str>>(mapping: &[S]) -> HashMap<String, Vec<String>> {
    mapping
        .iter()
        .map(|inst| {
            let (k, v) = inst.as_ref().split_once(':').unwrap();
            (k.to_string(), v.split(',').map(|s| s.to_string()).collect())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let mapping = parse(&[
            "RR:A,B,C", "A:D,E", "B:F,@", "C:G,H", "D:@", "E:@", "F:@", "G:@", "H:@",
        ]);
        assert_eq!("RRB@", part_one(&mapping));
    }
}

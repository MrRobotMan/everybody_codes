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

fn part_one(mapping: &HashMap<String, String>) -> String {
    get_paths(mapping).join("")
}

fn part_two() -> String {
    "Unsolved".into()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn get_paths(mapping: &HashMap<String, String>) -> Vec<String> {
    let mut paths: HashMap<usize, Vec<Vec<String>>> = HashMap::new();
    let mut queue = mapping
        .iter()
        .filter_map(|(k, v)| {
            if k.starts_with("@") {
                Some((v.as_str(), vec!["@"]))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    while let Some((node, steps)) = queue.pop() {
        let mut new_steps = steps.clone();
        new_steps.push(node);
        match mapping[node].as_str() {
            "RR" => {
                if steps[0] == "@" {
                    new_steps.push("RR");
                    let path = new_steps
                        .iter()
                        .rev()
                        .map(|s| s.to_string())
                        .collect::<Vec<_>>();
                    paths
                        .entry(new_steps.len())
                        .and_modify(|s| s.push(path.clone()))
                        .or_insert(vec![path]);
                }
            }
            parent => queue.push((parent, new_steps)),
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

fn parse<S: AsRef<str>>(mapping: &[S]) -> HashMap<String, String> {
    let mut ends = 0;
    mapping
        .iter()
        .flat_map(|inst| {
            let (k, v) = inst.as_ref().split_once(':').unwrap();
            v.split(',')
                .map(|s| {
                    (
                        if s == "@" {
                            ends += 1;
                            format!("{s}{ends}")
                        } else {
                            s.to_string()
                        },
                        k.to_string(),
                    )
                })
                .collect::<Vec<_>>()
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
        assert_eq!(vec!["RR", "B", "@"], get_paths(&mapping));
    }
}

use ebclib::read_lines;
use std::collections::HashMap;

fn main() {
    let input = read_lines("ebc2024/inputs/quest11.1.txt");
    println!(
        "Part 1: {}",
        get_termites(HashMap::from([("A".into(), 1)]), 4, &conversions(input))
    );

    let input = read_lines("ebc2024/inputs/quest11.2.txt");
    println!(
        "Part 2: {}",
        get_termites(HashMap::from([("Z".into(), 1)]), 10, &conversions(input))
    );

    let input = read_lines("ebc2024/inputs/quest11.3.txt");
    println!("Part 3: {}", part_three(conversions(input)));
}

fn part_three(conversions: HashMap<String, Vec<String>>) -> u64 {
    let res = conversions
        .keys()
        .map(|k| get_termites(HashMap::from([(k.clone(), 1)]), 20, &conversions))
        .collect::<Vec<_>>();
    res.iter().max().unwrap() - res.iter().min().unwrap()
}

fn conversions(notes: Vec<String>) -> HashMap<String, Vec<String>> {
    let mut res = HashMap::new();
    for line in notes {
        let (key, values) = line.split_once(':').unwrap();
        let values = values.split(',').map(|s| s.into()).collect();
        res.insert(key.into(), values);
    }
    res
}

fn get_termites(
    mut seed: HashMap<String, u64>,
    days: u64,
    conversions: &HashMap<String, Vec<String>>,
) -> u64 {
    for _ in 0..days {
        let mut next_day = HashMap::new();
        for (key, conversion) in conversions.iter() {
            for conv in conversion {
                let count = next_day.entry(conv.clone()).or_insert(0);
                *count += seed.get(key).unwrap_or(&0);
            }
        }
        seed = next_day;
    }
    seed.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = HashMap::from([
            ("A".into(), vec!["B".into(), "C".into()]),
            ("B".into(), vec!["C".into(), "A".into()]),
            ("C".into(), vec!["A".into()]),
        ]);
        let actual = conversions(vec!["A:B,C".into(), "B:C,A".into(), "C:A".into()]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_one() {
        let expected = 8;
        let rules = HashMap::from([
            ("A".into(), vec!["B".into(), "C".into()]),
            ("B".into(), vec!["C".into(), "A".into()]),
            ("C".into(), vec!["A".into()]),
        ]);
        let actual = get_termites(HashMap::from([("A".into(), 1)]), 4, &rules);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_initial_conditions() {
        let expected = 268815;
        let actual = part_three(HashMap::from([
            ("A".into(), vec!["B".into(), "C".into()]),
            ("B".into(), vec!["C".into(), "A".into(), "A".into()]),
            ("C".into(), vec!["A".into()]),
        ]));
        assert_eq!(expected, actual);
    }
}

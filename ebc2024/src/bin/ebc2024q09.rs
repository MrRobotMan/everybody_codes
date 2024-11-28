use ebclib::{read_lines, read_numbers};

fn main() {
    let input = read_numbers("ebc2024/inputs/quest09.1.txt");
    println!("Part 1: {}", part_one(&input));

    let input = read_numbers("ebc2024/inputs/quest09.2.txt");
    println!("Part 2: {}", part_two(&input));

    let _input = read_lines("ebc2024/inputs/quest09.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(sparkballs: &[u64]) -> u64 {
    let stamps = [1, 3, 5, 10];
    get_beetle_count(sparkballs, &stamps).iter().sum()
    // sparkballs
    //     .iter()
    //     .map(|s| get_beetle_count(*s, &stamps))
    //     .sum()
}

fn part_two(sparkballs: &[u64]) -> u64 {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    get_beetle_count(sparkballs, &stamps).iter().sum()
    // sparkballs
    //     .iter()
    //     .map(|s| get_beetle_count(*s, &stamps))
    //     .sum()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn get_beetle_count(sparkballs: &[u64], stamps: &[u64]) -> Vec<u64> {
    let mut counts = vec![vec![0; *sparkballs.iter().max().unwrap() as usize + 1]; stamps.len()];
    for sparkball in sparkballs {
        if counts[0][*sparkball as usize] > 0 {
            continue;
        }
        stamps
            .iter()
            .rev()
            .enumerate()
            .rev()
            .for_each(|(stamp_idx, stamp)| {
                (1..=*sparkball).for_each(|index| {
                    counts[stamp_idx][index as usize] = u64::MAX;
                    let mut keep = u64::MAX;
                    let mut drop = u64::MAX;
                    if let Some(v) = index.checked_sub(*stamp) {
                        keep = counts[stamp_idx][v as usize];
                        keep = keep.saturating_add(1);
                    }
                    if stamp_idx < (stamps.len() - 1) {
                        drop = counts[stamp_idx + 1][index as usize];
                    }
                    counts[stamp_idx][index as usize] = keep.min(drop);
                })
            });
    }
    sparkballs.iter().map(|s| counts[0][*s as usize]).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_count() {
        assert_eq!(
            3,
            get_beetle_count(&[19], &[1, 5, 6, 9]).iter().sum::<u64>()
        );
    }

    #[test]
    fn test_one() {
        let sparkballs = [2, 4, 7, 16];
        assert_eq!(10, part_one(&sparkballs));
    }

    #[test]
    fn test_two() {
        let sparkballs = [33, 41, 55, 99];
        assert_eq!(10, part_two(&sparkballs));
    }
}

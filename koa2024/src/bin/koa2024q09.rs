use puzlib::read_numbers;

fn main() {
    let input = read_numbers("koa2024/inputs/quest09.1.txt");
    println!("Part 1: {}", part_one(&input));

    let input = read_numbers("koa2024/inputs/quest09.2.txt");
    println!("Part 2: {}", part_two(&input));

    let input = read_numbers("koa2024/inputs/quest09.3.txt");
    println!("Part 3: {}", part_three(&input));
}

fn part_one(sparkballs: &[usize]) -> usize {
    let stamps = [1, 3, 5, 10];
    let values = get_beetle_count(sparkballs, &stamps);
    sparkballs.iter().map(|s| values[*s]).sum()
}

fn part_two(sparkballs: &[usize]) -> usize {
    let stamps = [1, 3, 5, 10, 15, 16, 20, 24, 25, 30];
    let values = get_beetle_count(sparkballs, &stamps);
    sparkballs.iter().map(|s| values[*s]).sum()
}

fn part_three(sparkballs: &[usize]) -> usize {
    let stamps = [
        1, 3, 5, 10, 15, 16, 20, 24, 25, 30, 37, 38, 49, 50, 74, 75, 100, 101,
    ];
    let values = get_beetle_count(sparkballs, &stamps);
    sparkballs
        .iter()
        .map(|sparkball| {
            (0..=50)
                .map(|delta| {
                    let left = sparkball / 2 + delta;
                    let right = sparkball - left;
                    values[left] + values[right]
                })
                .min()
                .unwrap()
        })
        .sum()
}

fn get_beetle_count(sparkballs: &[usize], stamps: &[usize]) -> Vec<usize> {
    let biggest = *sparkballs.iter().max().unwrap() + 1;
    let mut counts = vec![biggest; biggest];
    counts[0] = 0;
    stamps.iter().for_each(|stamp| {
        (*stamp..biggest).for_each(|idx| counts[idx] = counts[idx].min(counts[idx - *stamp] + 1))
    });
    counts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_count() {
        assert_eq!(3, get_beetle_count(&[19], &[1, 5, 6, 9])[19]);
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

    #[test]
    fn test_three() {
        let sparkballs = [156488, 352486, 546212];
        assert_eq!(10449, part_three(&sparkballs));
    }
}

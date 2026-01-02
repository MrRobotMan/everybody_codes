use ebclib::Median;
use puzlib::read_lines;

fn main() {
    let input = read_lines("koa2024/inputs/quest04.1.txt")
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 1: {}", part_one(&input));

    let input = read_lines("koa2024/inputs/quest04.2.txt")
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("Part 2: {}", part_one(&input));

    let mut input = read_lines("koa2024/inputs/quest04.3.txt")
        .iter()
        .map(|v| v.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    input.sort();
    println!("Part 3: {}", part_three(&input));
}

fn part_one(nails: &[i64]) -> i64 {
    let lowest = nails.iter().min().unwrap();
    nails.iter().map(|nail| nail - lowest).sum()
}

fn part_three(nails: &[i64]) -> i64 {
    let mid = nails.mid();
    mid.iter()
        .map(|m| nails.iter().map(|nail| nail.abs_diff(*m) as i64).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 10;
        let actual = part_one(&[3, 4, 7, 8]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let expected = 8;
        let actual = part_three(&[2, 4, 5, 6, 8]);
        assert_eq!(expected, actual);
    }
}

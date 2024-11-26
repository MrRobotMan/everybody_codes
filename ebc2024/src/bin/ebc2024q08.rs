use ebclib::read_lines;

fn main() {
    let input = read_lines("ebc2024/inputs/quest08.1.txt")[0]
        .parse::<i64>()
        .unwrap();
    println!("Part 1: {}", part_one(input));

    let _input = read_lines("ebc2024/inputs/quest08.2.txt");
    println!("Part 2: {}", part_two());

    let _input = read_lines("ebc2024/inputs/quest08.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(available: i64) -> i64 {
    let mut next_biggest = 0_i64;
    while next_biggest.pow(2) < available {
        next_biggest += 1;
    }
    (next_biggest.pow(2) - available) * (2 * next_biggest - 1)
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
        let expected = 21;
        let actual = part_one(13);
        assert_eq!(expected, actual);
    }
}

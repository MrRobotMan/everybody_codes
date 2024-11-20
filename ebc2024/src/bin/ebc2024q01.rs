use ebclib::read_line_chars;

fn main() {
    let input = read_line_chars("ebc2024/inputs/quest01.txt");
    println!("Part 1: {}", part_one(&input));
    println!("Part 2: {}", part_two());
    println!("Part 3: {}", part_three());
}

fn part_one(input: &[char]) -> u32 {
    input
        .iter()
        .map(|c| std::convert::Into::<Potion>::into(*c).value())
        .sum::<u32>()
}

fn part_two() -> String {
    "Unsolved".into()
}
fn part_three() -> String {
    "Unsolved".into()
}

enum Potion {
    A,
    B,
    C,
}

impl From<char> for Potion {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            _ => unreachable!("Unknown character {value}"),
        }
    }
}

impl Potion {
    fn value(&self) -> u32 {
        match self {
            Potion::A => 0,
            Potion::B => 1,
            Potion::C => 3,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_potions() {
        let potions = "ABBAC".chars().collect::<Vec<_>>();
        assert_eq!(5, part_one(&potions));
    }
}

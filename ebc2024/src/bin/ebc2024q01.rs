use ebclib::read_line_chars;

fn main() {
    let input1 = read_line_chars("ebc2024/inputs/quest01.1.txt");
    println!("Part 1: {}", part_one(&input1));
    let input2 = read_line_chars("ebc2024/inputs/quest01.2.txt");
    println!("Part 2: {}", part_two(&input2));
    println!("Part 3: {}", part_three());
}

fn part_one(input: &[char]) -> u32 {
    input
        .iter()
        .filter_map(|c| std::convert::Into::<Creature>::into(*c).value())
        .sum::<u32>()
}

fn part_two(input: &[char]) -> u32 {
    input
        .chunks(2)
        .map(|w| {
            let a: Creature = w[0].into();
            let b: Creature = w[1].into();
            match (a.value(), b.value()) {
                (None, Some(v)) | (Some(v), None) => v,
                (Some(v1), Some(v2)) => v1 + v2 + 2,
                (None, None) => 0,
            }
        })
        .sum::<u32>()
}
fn part_three() -> String {
    "Unsolved".into()
}

enum Creature {
    A,
    B,
    C,
    D,
    Empty,
}

impl From<char> for Creature {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'B' => Self::B,
            'C' => Self::C,
            'D' => Self::D,
            'x' => Self::Empty,
            _ => unreachable!("Unknown character {value}"),
        }
    }
}

impl Creature {
    fn value(&self) -> Option<u32> {
        match self {
            Creature::A => Some(0),
            Creature::B => Some(1),
            Creature::C => Some(3),
            Creature::D => Some(5),
            Creature::Empty => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let potions = "ABBAC".chars().collect::<Vec<_>>();
        assert_eq!(5, part_one(&potions));
    }

    #[test]
    fn test_part_two() {
        let potions = "AxBCDDCAxD".chars().collect::<Vec<_>>();
        assert_eq!(28, part_two(&potions));
    }
}

use ebclib::read_line_chars;

fn main() {
    let input1 = read_line_chars("ebc2024/inputs/quest01.1.txt");
    println!("Part 1: {}", part_one(&input1));
    let input2 = read_line_chars("ebc2024/inputs/quest01.2.txt");
    println!("Part 2: {}", part_two(&input2));
    let input3 = read_line_chars("ebc2024/inputs/quest01.3.txt");
    println!("Part 3: {}", part_three(&input3));
}

fn part_one(input: &[char]) -> u32 {
    potion_calculator(input, 1)
}

fn part_two(input: &[char]) -> u32 {
    potion_calculator(input, 2)
}
fn part_three(input: &[char]) -> u32 {
    potion_calculator(input, 3)
}

fn potion_calculator(creatures: &[char], mob_size: usize) -> u32 {
    creatures
        .chunks(mob_size)
        .map(|w| {
            let mob = w
                .iter()
                .filter_map(|c| std::convert::Into::<Creature>::into(*c).value())
                .collect::<Vec<_>>();
            mob.iter().sum::<u32>() + (mob.len().saturating_sub(1) * mob.len()) as u32
        })
        .sum::<u32>()
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
        let creatures = "ABBAC".chars().collect::<Vec<_>>();
        assert_eq!(5, part_one(&creatures));
    }

    #[test]
    fn test_part_two() {
        let creatures = "AxBCDDCAxD".chars().collect::<Vec<_>>();
        assert_eq!(28, part_two(&creatures));
    }

    #[test]
    fn test_part_three() {
        let creatures = "xBxAAABCDxCC".chars().collect::<Vec<_>>();
        assert_eq!(30, part_three(&creatures));
    }
}

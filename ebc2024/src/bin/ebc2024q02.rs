use ebclib::read_lines;
use regex::Regex;

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
    println!("Part 3: {}", part_three());
}

fn part_one() -> usize {
    let mut input = read_lines("ebc2024/inputs/quest02.txt");
    let inscription = input.pop().unwrap();
    let runes = Runes::new(input.pop().unwrap().split_once(':').unwrap().1);
    runes.find(inscription)
}

fn part_two() -> String {
    "Unsolved".into()
}
fn part_three() -> String {
    "Unsolved".into()
}

struct Runes {
    runes: Vec<String>,
}

impl Runes {
    fn new<T: AsRef<str>>(runes: T) -> Self {
        Self {
            runes: runes.as_ref().split(',').map(|s| s.to_string()).collect(),
        }
    }

    fn find<S: AsRef<str>>(&self, inscription: S) -> usize {
        self.runes
            .iter()
            .map(|r| {
                let re = Regex::new(r).unwrap();
                re.find_iter(inscription.as_ref()).count()
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let runes = Runes::new("THE,OWE,MES,ROD,HER");
        assert_eq!(
            4,
            runes.find("AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE")
        );
        assert_eq!(3, runes.find("THE FLAME SHIELDED THE HEART OF THE KINGS"));
        assert_eq!(2, runes.find("POWE PO WER P OWE R"));
        assert_eq!(3, runes.find("THERE IS THE END"));
    }
}

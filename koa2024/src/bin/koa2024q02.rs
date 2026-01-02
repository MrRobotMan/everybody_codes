use std::collections::{HashMap, HashSet};

use puzlib::read_lines;

fn main() {
    println!("Part 1: {}", part_one());
    println!("Part 2: {}", part_two());
    let inscriptions = read_lines("ebc2024/inputs/quest02.3.txt");
    println!("Part 3: {}", part_three(inscriptions));
}

fn part_one() -> usize {
    let mut input = read_lines("ebc2024/inputs/quest02.1.txt");
    let inscription = input.pop().unwrap();
    let runes = Runes::new(input.pop().unwrap().split_once(':').unwrap().1);
    runes.find_words(inscription)
}

fn part_two() -> usize {
    let mut inscriptions = read_lines("ebc2024/inputs/quest02.2.txt");
    let runes = Runes::new(inscriptions.remove(0).split_once(':').unwrap().1);
    inscriptions.iter().map(|i| runes.find_chars(i)).sum()
}
fn part_three(mut inscriptions: Vec<String>) -> usize {
    let runes = Runes::new(inscriptions.remove(0).split_once(':').unwrap().1);
    let mut matrix: HashMap<(i64, i64), char> = HashMap::new();
    for (row, line) in inscriptions.iter().enumerate() {
        for (col, char) in line.chars().enumerate() {
            matrix.insert((row as i64, col as i64), char);
        }
    }
    let rows = inscriptions.len();
    let cols = inscriptions.iter().map(|l| l.len()).max().unwrap();
    runes.find_chars_wrapped(matrix, (rows, cols))
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

    fn find_words<S: AsRef<str>>(&self, inscription: S) -> usize {
        let word = inscription.as_ref();
        self.runes
            .iter()
            .map(|r| {
                let mut hits = 0;
                for pos in 0..word.len() - r.len() + 1 {
                    if word[pos..].starts_with(r) {
                        hits += 1;
                    }
                }
                hits
            })
            .sum()
    }

    fn find_chars<S: AsRef<str>>(&self, inscription: S) -> usize {
        let mut runepos = HashSet::new();
        let line = inscription.as_ref();
        let rev = line.chars().rev().collect::<String>();

        for rune in &self.runes {
            if rune.len() > line.len() {
                continue;
            }
            for idx in 0..line.len() - rune.len() + 1 {
                if line[idx..].starts_with(rune) {
                    for pos in idx..idx + rune.len() {
                        runepos.insert(pos);
                    }
                }
                if rev[idx..].starts_with(rune) {
                    for pos in idx..idx + rune.len() {
                        runepos.insert(line.len() - pos - 1);
                    }
                }
            }
        }

        runepos.len()
    }
    fn find_chars_wrapped(&self, matrix: HashMap<(i64, i64), char>, size: (usize, usize)) -> usize {
        let mut runepos = HashSet::new();
        let rows = size.0 as i64;
        let cols = size.1 as i64;

        for rune in &self.runes {
            let rune_chars = rune.chars().collect::<Vec<_>>();
            for row in 0..rows {
                for col in 0..cols {
                    'dir: for dir in [(-1_i64, 0_i64), (0, 1), (1, 0), (0, -1)] {
                        let mut rloc = row;
                        let mut cloc = col;

                        for ch in &rune_chars {
                            if rloc < 0 || rloc >= rows {
                                continue 'dir;
                            }

                            if Some(ch) != matrix.get(&(rloc, cloc)) {
                                continue 'dir;
                            }
                            rloc += dir.0;
                            cloc += dir.1;
                            if cloc < 0 {
                                cloc = cols - 1;
                            } else if cloc >= cols {
                                cloc = 0;
                            }
                        }
                        rloc = row;
                        cloc = col;

                        for _ in 0..rune.len() {
                            runepos.insert((rloc, cloc));
                            rloc += dir.0;
                            cloc += dir.1;
                            if cloc < 0 {
                                cloc = cols - 1;
                            } else if cloc >= cols {
                                cloc = 0;
                            }
                        }
                    }
                }
            }
        }

        runepos.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let runes = Runes::new("THE,OWE,MES,ROD,HER");
        assert_eq!(
            4_usize,
            runes.find_words("AWAKEN THE POWER ADORNED WITH THE FLAMES BRIGHT IRE")
        );
        assert_eq!(
            3_usize,
            runes.find_words("THE FLAME SHIELDED THE HEART OF THE KINGS")
        );
        assert_eq!(2_usize, runes.find_words("POWE PO WER P OWE R"));
        assert_eq!(3_usize, runes.find_words("THERE IS THE END"));
    }

    #[test]
    fn test_two() {
        let runes = Runes::new("THE,OWE,MES,ROD,HER,QAQ");
        assert_eq!(5, runes.find_chars("UQAQAQU"))
    }

    #[test]
    fn test_three() {
        let text = vec![
            "WORDS:THE,OWE,MES,ROD,RODEO".to_string(),
            "HELWORLT".to_string(),
            "ENIGWDXL".to_string(),
            "TRODEOAL".to_string(),
        ];
        assert_eq!(10, part_three(text));
    }
}

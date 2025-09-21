use std::collections::HashMap;

use puzlib::{Dir, Graph, Vec2D, Weighted, dijkstra, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest13.1.txt");
    let chamber: Chamber = input.into();
    println!("Part 1: {}", chamber.traverse());

    let _input = read_lines("ebc2024/inputs/quest13.2.txt");
    println!("Part 2: Unsolved");

    let _input = read_lines("ebc2024/inputs/quest13.3.txt");
    println!("Part 3: Unsolved");
}

#[derive(Debug, Default)]
struct Chamber {
    chamber: HashMap<Vec2D<i64>, i64>,
    start: Vec2D<i64>,
    end: Vec2D<i64>,
    size: (usize, usize),
}

impl Chamber {
    fn traverse(&self) -> usize {
        let res = &dijkstra(&self.start, self).unwrap()[&self.end];
        res.0
    }

    fn show(&self) {
        for row in 0..self.size.0 {
            for col in 0..self.size.1 {
                match self.chamber.get(&Vec2D(row as i64, col as i64)) {
                    Some(n) => print!("{n}"),
                    None => print!("#"),
                }
            }
            println!();
        }
    }
}

impl Graph for Chamber {
    type Node = Vec2D<i64>;

    fn height(&self) -> usize {
        self.size.0
    }

    fn width(&self) -> usize {
        self.size.1
    }

    fn moves(&self, node: &Self::Node) -> Vec<Self::Node> {
        Dir::<i64>::cardinals()
            .iter()
            .filter_map(|n| {
                if self.chamber.contains_key(&(node + n)) {
                    Some(node + n)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_done(&self, node: &Self::Node) -> bool {
        node == &self.end
    }
}
impl Weighted for Chamber {
    fn weight(&self, cur: &Self::Node, next: &Self::Node) -> usize {
        let a = self.chamber[cur];
        let b = self.chamber[next];
        let cost = (a - b).abs().min(10 - (b - a).abs()) + 1;
        cost as usize
    }
}

impl<S: AsRef<str>> From<Vec<S>> for Chamber {
    fn from(value: Vec<S>) -> Self {
        let mut chamber = Chamber {
            size: (value.len(), value[0].as_ref().len()),
            ..Default::default()
        };
        for (row, line) in value.into_iter().enumerate() {
            for (col, ch) in line.as_ref().chars().enumerate() {
                let row = row as i64;
                let col = col as i64;
                match ch {
                    'E' => {
                        chamber.end = Vec2D(row, col);
                        chamber.chamber.insert(Vec2D(row, col), 0);
                    }
                    'S' => {
                        chamber.start = Vec2D(row, col);
                        chamber.chamber.insert(Vec2D(row, col), 0);
                    }
                    x if x.is_ascii_digit() => {
                        chamber
                            .chamber
                            .insert(Vec2D(row, col), (x as u8 - b'0') as i64);
                    }
                    _ => (),
                }
            }
        }
        chamber
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example1() {
        let expected = 28;
        let chamber: Chamber = r#"#######
#6769##
S50505E
#97434#
#######"#
            .lines()
            .collect::<Vec<_>>()
            .into();
        let actual = chamber.traverse();
        assert_eq!(expected, actual);
    }
}

use std::collections::HashMap;

use puzlib::{Dir, Graph, Vec2D, Weighted, dijkstra, read_lines};

fn main() {
    let input = read_lines("ebc2024/inputs/quest13.1.txt");
    let chamber: Chamber = input.into();
    println!("Part 1: {}", chamber.traverse());

    let input = read_lines("ebc2024/inputs/quest13.2.txt");
    let chamber: Chamber = input.into();
    println!("Part 2: {}", chamber.traverse());

    let input = read_lines("ebc2024/inputs/quest13.3.txt");
    let mut chamber: Chamber = input.into();
    chamber.from_end = true;
    println!("Part 3: {}", chamber.traverse());
}

#[derive(Debug, Default)]
struct Chamber {
    chamber: HashMap<Vec2D<i64>, i64>,
    start: Vec<Vec2D<i64>>,
    end: Vec2D<i64>,
    size: (usize, usize),
    from_end: bool,
}

impl Chamber {
    fn traverse(&self) -> usize {
        let mut res = usize::MAX;
        if self.from_end {
            if let Some((m, p)) = dijkstra(&self.end, self) {
                let found = p[p.len() - 1];
                res = m[&found]
            };
        } else {
            for start in self.start.iter() {
                if let Some((m, _)) = dijkstra(start, self) {
                    res = res.min(m[&self.end]);
                };
            }
        }
        res
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
        Dir::<i64>::cardinals(node)
            .iter()
            .filter_map(|n| {
                if let Some(next) = n
                    && self.chamber.contains_key(next)
                {
                    Some(*next)
                } else {
                    None
                }
            })
            .collect()
    }

    fn is_done(&self, node: &Self::Node) -> bool {
        if self.from_end {
            self.start.contains(node)
        } else {
            node == &self.end
        }
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
                        chamber.start.push(Vec2D(row, col));
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

    #[test]
    fn test_example3() {
        let expected = 14;
        let chamber: Chamber = r#"SSSSSSSSSSS
S674345621S
S###6#4#18S
S53#6#4532S
S5450E0485S
S##7154532S
S2##314#18S
S971595#34S
SSSSSSSSSSS"#
            .lines()
            .collect::<Vec<_>>()
            .into();
        let actual = chamber.traverse();
        assert_eq!(expected, actual);
    }
}

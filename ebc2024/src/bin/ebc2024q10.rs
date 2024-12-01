use std::collections::HashSet;

use ebclib::read_grid;

fn main() {
    let grid = read_grid("ebc2024/inputs/quest10.1.txt");
    println!("Part 1: {}", part_one(grid));

    let grids = read_grid("ebc2024/inputs/quest10.2.txt");
    println!("Part 2: {}", part_two(grids));

    let _grid = read_grid("ebc2024/inputs/quest10.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(grid: Grid<char>) -> String {
    find_word(&grid)
}

type Grid<T> = Vec<Vec<T>>;

fn part_two(grids: Grid<char>) -> usize {
    let mut shrines: Vec<Grid<char>> =
        vec![vec![]; (grids.len() / 9 + 1) * (grids[0].len() / 9 + 1)];
    let cols = grids[0].len() / 9 + 1;
    for (idx, row) in grids.iter().enumerate() {
        for (offset, ch) in row.chunks(9).enumerate() {
            shrines[(idx / 9) * cols + offset].push(
                ch.iter()
                    .filter(|c| **c != ' ')
                    .copied()
                    .collect::<Vec<char>>(),
            );
        }
    }
    shrines.iter().map(|s| power(find_word(s))).sum::<usize>()
}

fn part_three() -> String {
    "Unsolved".into()
}

fn power(word: String) -> usize {
    word.chars()
        .enumerate()
        .map(|(idx, ch)| (idx + 1) * (ch as u8 - b'A' + 1) as usize)
        .sum::<usize>()
}

fn find_word(grid: &Grid<char>) -> String {
    let ends = [0, 1, 6, 7];
    (2..6)
        .flat_map(|row| {
            (2..6)
                .flat_map(|col| {
                    let row_opts = ends.iter().map(|c| grid[row][*c]).collect::<HashSet<_>>();
                    let col_opts = ends.iter().map(|r| grid[*r][col]).collect::<HashSet<_>>();
                    row_opts
                        .intersection(&col_opts)
                        .copied()
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let shrine = read_grid(
            "**PCBS**
**RLNW**
BV....PT
CR....HZ
FL....JW
SG....MN
**FTZV**
**GMJH**",
        );
        let expected = "PTBVRCZHFLJWGMNS";
        let actual = part_one(shrine);
        assert_eq!(expected, actual);
        assert_eq!(1851, power(actual))
    }
}

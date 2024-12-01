use std::collections::HashSet;

use ebclib::read_grid;

fn main() {
    let grid = read_grid("ebc2024/inputs/quest10.1.txt");
    println!("Part 1: {}", part_one(grid));

    let _grid = read_grid("ebc2024/inputs/quest10.2.txt");
    println!("Part 2: {}", part_two());

    let _grid = read_grid("ebc2024/inputs/quest10.3.txt");
    println!("Part 3: {}", part_three());
}

fn part_one(grid: Vec<Vec<char>>) -> String {
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
        let grid = read_grid(
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
        let actual = part_one(grid);
        assert_eq!(expected, actual);
    }
}

use std::collections::HashSet;

use puzlib::read_grid;

fn main() {
    let grid = read_grid("koa2024/inputs/quest10.1.txt");
    println!("Part 1: {}", part_one(grid));

    let grids = read_grid("koa2024/inputs/quest10.2.txt");
    println!("Part 2: {}", part_two(grids));

    let grids = read_grid("koa2024/inputs/quest10.3.txt");
    println!("Part 3: {}", part_three(grids));
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
    shrines.iter().map(|s| power(&find_word(s))).sum::<usize>()
}

fn part_three(mut grids: Grid<char>) -> usize {
    let rows = grids.len();
    let cols = grids[0].len();
    let mut corners = (0..rows - 2)
        .step_by(6)
        .flat_map(|row| {
            (0..cols - 2)
                .step_by(6)
                .map(|col| ((row, col), 0))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    corners.iter_mut().for_each(|c| {
        solve_grid(&mut grids, &c.0);
    });
    loop {
        let mut updated = false;
        for corner in corners.iter_mut() {
            if corner.1 > 0 {
                continue;
            }
            if let Some(p) = solve_grid(&mut grids, &corner.0) {
                corner.1 = p;
                updated = true;
            }
        }
        if !updated {
            break;
        }
    }
    corners.iter().map(|c| c.1).sum()
}

fn solve_grid(grid: &mut Grid<char>, loc: &(usize, usize)) -> Option<usize> {
    let ends = [0, 1, 6, 7];
    // Find and replace dots if possible.
    (loc.0 + 2..loc.0 + 6).for_each(|row| {
        if grid[row].contains(&'.') {
            (loc.1 + 2..loc.1 + 6).for_each(|col| {
                let row_opts = ends
                    .iter()
                    .map(|c| grid[row][*c + loc.1])
                    .collect::<HashSet<_>>();
                let col_opts = ends
                    .iter()
                    .map(|r| grid[*r + loc.0][col])
                    .collect::<HashSet<_>>();
                let intersect = row_opts.intersection(&col_opts).collect::<Vec<_>>();
                if intersect.len() == 1 {
                    grid[row][col] = *intersect[0];
                }
            })
        }
    });
    for row in 2..6 {
        for col in 2..6 {
            if grid[loc.0 + row][loc.1 + col] != '.' {
                continue;
            }
            let mut unique = HashSet::new();
            let mut question = (0, 0);
            for idx in &ends {
                match grid[loc.0 + idx][loc.1 + col] {
                    '?' => question = (loc.0 + idx, loc.1 + col),
                    c => {
                        unique.insert(c);
                    }
                }
                match grid[loc.0 + row][loc.1 + idx] {
                    '?' => question = (loc.0 + row, loc.1 + idx),
                    c => {
                        unique.insert(c);
                    }
                }
            }
            for idx in 2..6 {
                unique.remove(&grid[loc.0 + idx][loc.1 + col]);
                unique.remove(&grid[loc.0 + row][loc.1 + idx]);
            }
            if unique.len() == 1 {
                grid[loc.0 + row][loc.1 + col] = *unique.iter().next().unwrap();
                grid[question.0][question.1] = *unique.iter().next().unwrap();
            }
        }
    }
    let word = find_word(
        &(loc.0..loc.0 + 8)
            .map(|row| {
                (loc.1..loc.1 + 8)
                    .map(|col| grid[row][col])
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );
    if !word.is_empty() {
        Some(power(&word))
    } else {
        None
    }
}

fn power(word: &str) -> usize {
    word.chars()
        .enumerate()
        .map(|(idx, ch)| (idx + 1) * (ch as u8 - b'A' + 1) as usize)
        .sum::<usize>()
}

fn find_word(grid: &Grid<char>) -> String {
    let ends = [0, 1, 6, 7];
    let res = (2..6)
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
        .collect::<String>();
    if res.len() != 16 || res.contains('?') {
        String::new()
    } else {
        res
    }
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
        assert_eq!(1851, power(&actual))
    }

    #[test]
    fn test_three() {
        let grids = read_grid(
            "**XFZB**DCST**
**LWQK**GQJH**
?G....WL....DQ
BS....H?....CN
P?....KJ....TV
NM....Z?....SG
**NSHM**VKWZ**
**PJGV**XFNL**
WQ....?L....YS
FX....DJ....HV
?Y....WM....?J
TJ....YK....LP
**XRTK**BMSP**
**DWZN**GCJV**",
        );
        let expected = 3889;
        let actual = part_three(grids);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_failed_grid() {
        let mut grid = vec![
            "**NSHM**".chars().collect::<Vec<_>>(),
            "**PJGV**".chars().collect::<Vec<_>>(),
            "WQ....?L".chars().collect::<Vec<_>>(),
            "FX....DJ".chars().collect::<Vec<_>>(),
            "?Y....WM".chars().collect::<Vec<_>>(),
            "TJ....YK".chars().collect::<Vec<_>>(),
            "**XRTK**".chars().collect::<Vec<_>>(),
            "**DWZN**".chars().collect::<Vec<_>>(),
        ];
        let expected = "";
        let actual = find_word(&grid);
        assert_eq!(expected, actual);
        assert_eq!(0, power(&actual));
        assert_eq!(None, solve_grid(&mut grid, &(0, 0)));
    }

    #[test]
    fn test_solved_grid() {
        let mut grid = vec![
            "**XFZB**".chars().collect::<Vec<_>>(),
            "**LWQK**".chars().collect::<Vec<_>>(),
            "?G....WL".chars().collect::<Vec<_>>(),
            "BS....H?".chars().collect::<Vec<_>>(),
            "P?....KJ".chars().collect::<Vec<_>>(),
            "NM....Z?".chars().collect::<Vec<_>>(),
            "**NSHM**".chars().collect::<Vec<_>>(),
            "**PJGV**".chars().collect::<Vec<_>>(),
        ];
        let expected = vec![
            "**XFZB**".chars().collect::<Vec<_>>(),
            "**LWQK**".chars().collect::<Vec<_>>(),
            "VGLWGVWL".chars().collect::<Vec<_>>(),
            "BSXSHBHX".chars().collect::<Vec<_>>(),
            "PQPJQKKJ".chars().collect::<Vec<_>>(),
            "NMNFZMZF".chars().collect::<Vec<_>>(),
            "**NSHM**".chars().collect::<Vec<_>>(),
            "**PJGV**".chars().collect::<Vec<_>>(),
        ];
        let actual = solve_grid(&mut grid, &(0, 0));
        assert_eq!(Some(1900), actual);
        assert_eq!(expected, grid);
    }
}

use std::collections::VecDeque;

use puzlib::read_lines;

fn main() {
    let blocks = read_lines("ebc2024/inputs/quest08.1.txt")[0]
        .parse::<i64>()
        .unwrap();
    println!("Part 1: {}", part_one(blocks));

    let priests = read_lines("ebc2024/inputs/quest08.2.txt")[0]
        .parse::<i64>()
        .unwrap();
    println!("Part 2: {}", part_two(priests, 1111, 20240000));

    let priests = read_lines("ebc2024/inputs/quest08.3.txt")[0]
        .parse::<u64>()
        .unwrap();
    println!("Part 3: {}", part_three(priests, 10, 202400000));
}

fn part_one(available: i64) -> i64 {
    let mut next_biggest = 0_i64;
    while next_biggest.pow(2) < available {
        next_biggest += 1;
    }
    (next_biggest.pow(2) - available) * (2 * next_biggest - 1)
}

fn part_two(priests: i64, acolytes: i64, blocks: i64) -> i64 {
    let mut width = 1;
    let mut thickness = 1;
    let mut total = 1;
    while total < blocks {
        // Assuming multiplications will become large, we're distributing the modulo.
        // (a * b) % n == ((a % n) * (b % n)) %n
        thickness = ((thickness % acolytes) * (priests % acolytes)) % acolytes;
        width += 2;
        total += width * thickness;
        #[cfg(test)]
        println!("Adding {thickness} * {width}. Total needed {total}");
    }
    (total - blocks) * width
}

fn part_three(priests: u64, acolytes: u64, blocks: u64) -> u64 {
    let mut heights = VecDeque::new();
    let mut width = 1;
    let mut thickness = 1;
    heights.push_front(1);
    #[cfg(test)]
    println!("{heights:?}");
    let mut total = 1;
    loop {
        // Assuming multiplications will become large, we're distributing the modulo.
        // (a * b) % n == ((a % n) * (b % n)) %n
        thickness = ((thickness % acolytes) * (priests % acolytes)) % acolytes + acolytes;
        width += 2;
        heights.iter_mut().for_each(|h| *h += thickness);
        heights.push_front(thickness);
        heights.push_back(thickness);
        total += width * thickness;
        let removed = heights
            .iter()
            .skip(1)
            .take(heights.len() - 2)
            .map(|&h| (priests % acolytes * width % acolytes * h % acolytes) % acolytes)
            .collect::<Vec<_>>();
        let removed = removed.iter().sum::<u64>();
        if (total - removed) > blocks {
            return total - removed - blocks;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let expected = 21;
        let actual = part_one(13);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_two() {
        let expected = 27;
        let actual = part_two(3, 5, 50);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_three() {
        let expected = 2;
        let actual = part_three(2, 5, 160);
        assert_eq!(expected, actual);
    }
}

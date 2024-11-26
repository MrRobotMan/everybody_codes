use ebclib::read_lines;

fn main() {
    let blocks = read_lines("ebc2024/inputs/quest08.1.txt")[0]
        .parse::<i64>()
        .unwrap();
    println!("Part 1: {}", part_one(blocks));

    let priests = read_lines("ebc2024/inputs/quest08.2.txt")[0]
        .parse::<i64>()
        .unwrap();
    println!("Part 2: {}", part_two(priests, 1111, 20240000));

    let _input = read_lines("ebc2024/inputs/quest08.3.txt");
    println!("Part 3: {}", part_three());
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

fn part_three() -> String {
    "Unsolved".into()
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
}

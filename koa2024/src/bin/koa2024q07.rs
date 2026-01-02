use num::Integer;
use std::{
    collections::{HashMap, HashSet},
    sync::LazyLock,
};

use puzlib::{Dir, Permutations, Vec2D, read_lines};

fn main() {
    let plans = read_lines("ebc2024/inputs/quest07.1.txt")
        .iter()
        .map(|l| {
            let (name, actions) = l.split_once(':').unwrap();
            Device::new(name, actions)
        })
        .collect::<Vec<Device>>();
    println!("Part 1: {}", part_one(plans));

    let plans = read_lines("ebc2024/inputs/quest07.2.txt")
        .iter()
        .map(|l| {
            let (name, actions) = l.split_once(':').unwrap();
            Device::new(name, actions)
        })
        .collect::<Vec<Device>>();
    let track = parse_track(
        "S-=++=-==++=++=-=+=-=+=+=--=-=++=-==++=-+=-=+=-=+=+=++=-+==++=++=-=-=--
-                                                                     -
=                                                                     =
+                                                                     +
=                                                                     +
+                                                                     =
=                                                                     =
-                                                                     -
--==++++==+=+++-=+=-=+=-+-=+-=+-=+=-=+=--=+++=++=+++==++==--=+=++==+++-",
    );
    println!("Part 2: {}", part_two(plans, track));

    let plans = read_lines("ebc2024/inputs/quest07.3.txt")
        .iter()
        .map(|l| {
            let (name, actions) = l.split_once(':').unwrap();
            Device::new(name, actions)
        })
        .collect::<Vec<Device>>();
    let track = parse_track(
        "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-",
    );
    println!("Part 3: {}", part_three(plans, track));
}

fn part_one(mut action_plans: Vec<Device>) -> String {
    action_plans
        .iter_mut()
        .for_each(|device| (0..10).for_each(|_| device.enter_segment()));
    action_plans.sort_by(|a, b| b.score.cmp(&a.score));
    action_plans
        .iter()
        .map(|device| device.name.as_str())
        .collect::<String>()
}

fn part_two(mut action_plans: Vec<Device>, track: Vec<char>) -> String {
    (0..10).for_each(|_| {
        action_plans
            .iter_mut()
            .for_each(|device| device.lap(&track))
    });
    action_plans.sort_by(|a, b| b.score.cmp(&a.score));
    action_plans
        .iter()
        .map(|device| device.name.as_str())
        .collect::<String>()
}

fn part_three(mut action_plans: Vec<Device>, track: Vec<char>) -> usize {
    // Only need to calculate enough laps that the LCM of the laps and track length is
    // greater than the total number of laps.
    let laps_to_calculate = factors(2024)
        .iter()
        .filter_map(|f| {
            if Integer::lcm(f, &track.len()) > 2024 {
                Some(*f)
            } else {
                None
            }
        })
        .min()
        .unwrap_or(2024);
    (0..laps_to_calculate).for_each(|_| action_plans[0].lap(&track));
    let competitor = action_plans[0].score;
    let plans = action_plans[0].actions.permutations();
    plans
        .map(|p| {
            let mut d = Device {
                power: 10,
                actions: p.clone(),
                ..Default::default()
            };
            (0..laps_to_calculate).for_each(|_| d.lap(&track));
            if d.score > competitor { 1 } else { 0 }
        })
        .sum()
}

fn factors(mut number: usize) -> HashSet<usize> {
    let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43]; // Primes below sqsrt(2024)
    let mut found = HashSet::new();
    for prime in primes {
        while number.is_multiple_of(prime) {
            found.insert(prime);
            number /= prime;
        }
    }
    found
}

#[derive(Debug, Default)]
struct Device {
    name: String,
    power: usize,
    actions: Vec<char>,
    step: usize,
    score: usize,
}

impl Device {
    fn new<S: AsRef<str>>(name: S, actions: S) -> Self {
        Self {
            name: name.as_ref().into(),
            power: 10,
            actions: actions.as_ref().chars().filter(|ch| *ch != ',').collect(),
            ..Default::default()
        }
    }

    fn enter_segment(&mut self) {
        match self.actions[self.step] {
            '+' => self.power += 1,
            '-' => self.power = self.power.saturating_sub(1),
            '=' => (),
            c => unreachable!("Unknown action {c}"),
        }
        self.step_forward();
        self.score += self.power;
    }

    fn lap(&mut self, track: &[char]) {
        for ch in track {
            match *ch {
                '+' => {
                    self.power += 1;
                    self.step_forward();
                    self.score += self.power;
                }
                '-' => {
                    self.power -= 1;
                    self.step_forward();
                    self.score += self.power;
                }
                '=' | 'S' => {
                    self.enter_segment();
                }
                _ => unreachable!("Unknown command {ch}"),
            }
        }
    }

    fn step_forward(&mut self) {
        self.step = (self.step + 1) % self.actions.len();
    }
}

static DIRS: LazyLock<[Vec2D<i64>; 4]> = LazyLock::new(|| {
    Dir::<i64>::cardinals(&Vec2D(0, 0))
        .iter()
        .map(|d| d.unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
});

fn parse_track<S: AsRef<str>>(track: S) -> Vec<char> {
    let lines = track
        .as_ref()
        .lines()
        .enumerate()
        .flat_map(|(r, l)| {
            l.chars()
                .enumerate()
                .filter_map(|(c, ch)| {
                    if ch != ' ' {
                        Some(((r as i64, c as i64), ch))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<HashMap<(i64, i64), char>>();
    let mut row = 0;
    let mut col = 1;
    let mut offset = 1;
    let mut track = vec![];
    loop {
        match lines.get(&(row, col)) {
            Some(ch) => {
                track.push(*ch);
                if *ch == 'S' {
                    break;
                }
            }
            None => {
                row -= DIRS[offset].0;
                col -= DIRS[offset].1;
                offset = (offset + 1) % 4;
                if !lines.contains_key(&(row + DIRS[offset].0, col + DIRS[offset].1)) {
                    offset = (offset + 2) % 4;
                }
            }
        }
        row += DIRS[offset].0;
        col += DIRS[offset].1;
    }
    track
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let devices = "A:+,-,=,=
        B:+,=,-,+
        C:=,-,+,+
        D:=,=,=,+"
            .split('\n')
            .map(|l| {
                let (name, actions) = l.trim().split_once(':').unwrap();
                Device::new(name, actions)
            })
            .collect::<Vec<Device>>();
        assert_eq!("BDCA", part_one(devices));
    }

    #[test]
    fn test_two() {
        let mut devices = "A:+,-,=,=
        B:+,=,-,+
        C:=,-,+,+
        D:=,=,=,+"
            .split('\n')
            .map(|l| {
                let (name, actions) = l.trim().split_once(':').unwrap();
                Device::new(name, actions)
            })
            .collect::<Vec<Device>>();
        let track = parse_track("S+===\n-   +\n=+=-+");
        devices.iter_mut().for_each(|device| device.lap(&track));
        assert_eq!(
            vec![129, 148, 154, 158],
            devices.iter().map(|d| d.score).collect::<Vec<_>>()
        );
        devices.iter_mut().for_each(|d| {
            d.score = 0;
            d.step = 0;
            d.power = 0;
        });
        assert_eq!("DCBA", part_two(devices, track));
    }

    #[test]
    fn test_make_track() {
        assert_eq!(
            parse_track("S+===\n-   +\n=+=-+"),
            vec!['+', '=', '=', '=', '+', '+', '-', '=', '+', '=', '-', 'S']
        );
    }

    #[test]
    fn test_make_bent_track() {
        assert_eq!(
            parse_track(
                "S+= ===
- +++ +
-     +
=+=-+=="
            ),
            vec![
                '+', '=', '+', '+', '+', '=', '=', '=', '+', '+', '=', '=', '+', '-', '=', '+',
                '=', '-', '-', 'S'
            ]
        );
    }

    #[test]
    fn test_make_big_track() {
        let track = parse_track(
            "S+= +=-== +=++=     =+=+=--=    =-= ++=     +=-  =+=++=-+==+ =++=-=-=--
- + +   + =   =     =      =   == = - -     - =  =         =-=        -
= + + +-- =-= ==-==-= --++ +  == == = +     - =  =    ==++=    =++=-=++
+ + + =     +         =  + + == == ++ =     = =  ==   =   = =++=
= = + + +== +==     =++ == =+=  =  +  +==-=++ =   =++ --= + =
+ ==- = + =   = =+= =   =       ++--          +     =   = = =--= ==++==
=     ==- ==+-- = = = ++= +=--      ==+ ==--= +--+=-= ==- ==   =+=    =
-               = = = =   +  +  ==+ = = +   =        ++    =          -
-               = + + =   +  -  = + = = +   =        +     =          -
--==++++==+=+++-= =-= =-+-=  =+-= =-= =--   +=++=+++==     -=+=++==+++-",
        );
        let expected =
            "+=+++===-+++++=-==+--+=+===-++=====+--===++=-==+=++====-==-===+=+=--==++=+========-==\
=====++--+++=-++=-+=+==-=++=--+=-====++--+=-==++======+=++=-+==+=-==++=-=-=---++=-=++\
==++===--==+===++===---+++==++=+=-=====+==++===--==-==+++==+++=++=+===--==++--===+===\
==-=++====-+=-+--=+++=-+-===++====+++--=++====+=-=+===+=====-+++=+==++++==----=+=+=-S"
                .chars()
                .collect::<Vec<_>>();
        assert_eq!(expected, track);
    }

    #[test]
    fn test_factors() {
        assert_eq!(HashSet::from([3, 5]), factors(15));
        assert_eq!(HashSet::from([2]), factors(16));
    }
}

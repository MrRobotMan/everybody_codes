use ebclib::read_lines;

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

    let _input = read_lines("ebc2024/inputs/quest07.3.txt");
    println!("Part 3: {}", part_three());
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

fn part_three() -> String {
    "Unsolved".into()
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

fn parse_track<S: AsRef<str>>(track: S) -> Vec<char> {
    let lines = track
        .as_ref()
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let bottom = lines.len() - 1;
    let mut last_leg = vec![];
    let mut track = vec![];
    for (idx, line) in lines.into_iter().enumerate() {
        if idx == 0 {
            track.append(&mut line[1..].to_vec());
        } else if idx == bottom {
            track.append(&mut line.iter().rev().copied().collect::<Vec<_>>());
        } else {
            last_leg.push(line[0]);
            track.push(line[line.len() - 1]);
        }
    }
    track.append(&mut last_leg);
    track.push('S');
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
        assert_eq!("DCBA", part_two(devices, track));
    }

    #[test]
    fn test_make_track() {
        assert_eq!(
            parse_track("S+===\n-   +\n=+=-+"),
            vec!['+', '=', '=', '=', '+', '+', '-', '=', '+', '=', '-', 'S']
        );
    }
}

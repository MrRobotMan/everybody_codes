fn main() {
    let mut res = vec![];
    for day in 1..=12 {
        if let Ok(out) = std::process::Command::new("cargo")
            .args(["run", "--bin", &format!("ebc2024q{day:02}")])
            .output()
        {
            res.push(format!("2024 Quest {day:02}").as_bytes().to_vec());
            res.push(out.stdout);
        } else {
            res.push(
                format!("Failed to run 2024 quest {day:02}")
                    .as_bytes()
                    .to_vec(),
            );
        };
    }
    for day in res.iter() {
        println!("{}", str::from_utf8(day).unwrap());
    }
}

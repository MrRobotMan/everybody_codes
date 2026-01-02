use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use std::{
    env,
    error::Error,
    fs::{self, File, create_dir},
    io::{self, BufReader, Read},
    path::PathBuf,
};
use toml_edit::{ArrayOfTables, DocumentMut, Item, Table, value};

fn main() {
    let Some((year, quest, triple)) = get_args() else {
        {
            println!("Invalid arguments. Please supply year and quest.");
            return;
        }
    };

    match update_cargo(year, quest, &triple) {
        Ok(()) => println!("Updated {year} cargo."),
        Err(e) => println!("{e}"),
    };
    match create_quest(year, quest, &triple) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    }
    match update_bacon(year, quest, &triple) {
        Ok(_) => println!("Bacon updated."),
        Err(e) => println!("{e}"),
    }
}

fn create_quest(year: i32, quest: u32, triple: &str) -> io::Result<String> {
    let bin = PathBuf::from(format!("{triple}{year}/src/bin/ebc{year}q{quest:02}.rs"));
    if bin.exists() {
        return Ok(format!("{year} quest {quest} already exists. Skipping."));
    }
    if let Some(bin_dir) = bin.parent()
        && !bin_dir.exists()
    {
        let _ = create_dir(bin_dir);
    }
    let template = format!(
        r#"use puzlib::read_lines;
    
fn main() {{
    let _input = puzlib::FileReader::new("{triple}{year}/inputs/everbody_codes_e{year}_q{quest:02}_p1.txt");
    println!("Part 1: {{}}", part_one());

    let _input = puzlib::FileReader::new("{triple}{year}/inputs/everbody_codes_e{year}_q{quest:02}_p2.txt");
    println!("Part 2: {{}}", part_two());

    let _input = puzlib::FileReader::new("{triple}{year}/inputs/everbody_codes_e{year}_q{quest:02}_p3.txt");
    println!("Part 3: {{}}", part_three());
    }}

fn part_one() -> String {{
    "Unsolved".into()
}}

fn part_two() -> String {{
    "Unsolved".into()
}}

fn part_three() -> String {{
    "Unsolved".into()
}}

#[cfg(test)]
mod tests {{
    use super::*;

    #[test]
    fn test_one() {{
        let expected = 1;
        let actual = 0;
        assert_eq!(expected, actual);
    }}
}}
    "#
    );
    fs::write(bin, template)?;
    Ok(format!("Created {year} quest {quest}."))
}

/// Update the year's cargo file for the new binary.
fn update_cargo(year: i32, quest: u32, triple: &str) -> Result<(), Box<dyn Error>> {
    let cargo_file = format!("{triple}{year}/Cargo.toml");
    let mut cargo = get_existing_file(&cargo_file)?.parse::<DocumentMut>()?;
    let mut new_table = Table::new();
    new_table["name"] = value(format!("{triple}{year}q{quest:02}"));
    let bin = cargo
        .entry("bin")
        .or_insert(Item::ArrayOfTables(ArrayOfTables::new()))
        .as_array_of_tables_mut()
        .unwrap();
    if !bin.iter().any(|t| *t.to_string() == new_table.to_string()) {
        bin.push(new_table);
    };
    fs::write(cargo_file, cargo.to_string())?;
    Ok(())
}

/// Update bacon to call the binary when using run.
fn update_bacon(year: i32, quest: u32, triple: &str) -> io::Result<()> {
    let mut bacon = get_existing_file("bacon.toml")?
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    for line in bacon.iter_mut() {
        if line.contains("--package") {
            *line = format!(r#"    "--package", "{triple}{year}","#);
        }
        if line.contains("--bin") {
            *line = format!(r#"    "--bin", "{triple}{year}q{quest:02}","#);
        }
    }
    fs::write("bacon.toml", bacon.join("\n"))
}

/// Read the existing file.
fn get_existing_file(cargo_file: &str) -> io::Result<String> {
    let cargo = File::open(cargo_file)?;
    let mut reader = BufReader::new(cargo);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

/// Gather the commandline arguments.
fn get_args() -> Option<(i32, u32, String)> {
    let mut args = env::args();
    let triple = args.next().expect("Need a year triple");
    match args.len() {
        1 => {
            // Rework this. Not sure when the event will start each year.
            // Runs first Monday of November for 20 weekdays.
            let east_coast = FixedOffset::west_opt(5 * 60 * 60).unwrap();
            let today = Utc::now().with_timezone(&east_coast);
            if today
                < east_coast
                    .with_ymd_and_hms(today.year(), 12, 1, 0, 0, 0)
                    .unwrap()
            {
                Some((today.year(), 1, triple))
            } else {
                Some((today.year(), today.day(), triple))
            }
        }
        2 => {
            args.next();
            let year = args.next();
            Some((year.unwrap().parse().unwrap(), 1, triple))
        }
        3 => {
            args.next();
            let year = args.next();
            let quest = args.next();
            Some((
                year.unwrap().parse().unwrap(),
                quest.unwrap().parse().unwrap(),
                triple,
            ))
        }
        _ => None,
    }
}

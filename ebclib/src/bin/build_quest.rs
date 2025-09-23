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
    let Some((year, quest)) = get_args() else {
        {
            println!("Invalid arguments. Please supply year and quest.");
            return;
        }
    };

    match update_cargo(year, quest) {
        Ok(()) => println!("Updated {year} cargo."),
        Err(e) => println!("{e}"),
    };
    match create_quest(year, quest) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    }
    match update_bacon(year, quest) {
        Ok(_) => println!("Bacon updated."),
        Err(e) => println!("{e}"),
    }
}

fn create_quest(year: i32, quest: u32) -> io::Result<String> {
    let bin = PathBuf::from(format!("ebc{year}/src/bin/ebc{year}q{quest:02}.rs"));
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
    let _input = read_lines("ebc{year}/inputs/quest{quest:02}.1.txt");
    println!("Part 1: {{}}", part_one());

    let _input = read_lines("ebc{year}/inputs/quest{quest:02}.2.txt");
    println!("Part 2: {{}}", part_two());

    let _input = read_lines("ebc{year}/inputs/quest{quest:02}.3.txt");
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
fn update_cargo(year: i32, quest: u32) -> Result<(), Box<dyn Error>> {
    let cargo_file = format!("ebc{year}/Cargo.toml");
    let mut cargo = get_existing_file(&cargo_file)?.parse::<DocumentMut>()?;
    let mut new_table = Table::new();
    new_table["name"] = value(format!("ebc{year}q{quest:02}"));
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
fn update_bacon(year: i32, quest: u32) -> io::Result<()> {
    let mut bacon = get_existing_file("bacon.toml")?
        .split('\n')
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    for line in bacon.iter_mut() {
        if line.contains("--package") {
            *line = format!(r#"    "--package", "ebc{year}","#);
        }
        if line.contains("--bin") {
            *line = format!(r#"    "--bin", "ebc{year}q{quest:02}","#);
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
fn get_args() -> Option<(i32, u32)> {
    let mut args = env::args();
    match args.len() {
        1 => {
            // Rework this. Not sure when the event will start each year.
            let east_coast = FixedOffset::west_opt(5 * 60 * 60).unwrap();
            let today = Utc::now().with_timezone(&east_coast);
            if today
                < east_coast
                    .with_ymd_and_hms(today.year(), 12, 1, 0, 0, 0)
                    .unwrap()
            {
                Some((today.year(), 1))
            } else {
                Some((today.year(), today.day()))
            }
        }
        2 => {
            args.next();
            let year = args.next();
            Some((year.unwrap().parse().unwrap(), 1))
        }
        3 => {
            args.next();
            let year = args.next();
            let quest = args.next();
            Some((
                year.unwrap().parse().unwrap(),
                quest.unwrap().parse().unwrap(),
            ))
        }
        _ => None,
    }
}

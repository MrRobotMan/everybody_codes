use chrono::{Datelike, FixedOffset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, create_dir, File},
    io::{self, BufReader, Read},
    path::PathBuf,
};

fn main() {
    let Some((year, quest)) = get_args() else {
        {
            println!("Invalid arguments. Please supply year and quest.");
            return;
        }
    };

    match update_cargo(year, quest) {
        Err(e) => println!("{e}"),
        Ok(()) => println!("Updated {year} cargo."),
    };
    match create_quest(year, quest) {
        Ok(s) => println!("{s}"),
        Err(e) => println!("{e}"),
    }
}

fn create_quest(year: i32, quest: u32) -> io::Result<String> {
    let bin = PathBuf::from(format!("ebc{year}/src/bin/ebc{year}q{quest:02}.rs"));
    if bin.exists() {
        return Ok(String::from("{year} quest {day} already exists. Skipping."));
    }
    if let Some(bin_dir) = bin.parent() {
        if !bin_dir.exists() {
            let _ = create_dir(bin_dir);
        }
    }
    let template = format!(
        r#"use ebclib::read_lines;
    
fn main() {{
    let input = read_lines("ebc{year}/inputs/quest{quest:02}.txt");
    println!("{{}}", part_one());
    println!("{{}}", part_two());
    println!("{{}}", part_three());
    }}

fn part_one() {{
    todo!()
}}

fn part_two() {{
    todo!()
}}
fn part_three() {{
    todo!()
}}
    "#
    );
    fs::write(bin, template)?;
    Ok(format!("Created {year} quest {quest}."))
}

/// Update the year's cargo file for the new binary.
fn update_cargo(year: i32, quest: u32) -> Result<(), Box<dyn Error>> {
    let cargo_file = format!("ebc{year}/Cargo.toml");
    let mut cargo: Cargo = toml::from_str(&get_existing_cargo_text(&cargo_file)?)?;
    let new_quest = Bin {
        name: format!("ebc{year}q{quest:02}"),
    };
    match cargo.bin.as_mut() {
        Some(b) => {
            if !b.contains(&new_quest) {
                b.push(new_quest);
            }
        }
        None => cargo.bin = Some(vec![new_quest]),
    }
    let cargo_str = toml::to_string_pretty(&cargo)?;
    fs::write(cargo_file, cargo_str)?;
    Ok(())
}

/// Read the existing cargo file.
fn get_existing_cargo_text(cargo_file: &str) -> io::Result<String> {
    let cargo = File::open(cargo_file)?;
    let mut reader = BufReader::new(cargo);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer)
}

#[derive(Debug, Deserialize, Serialize)]
struct Cargo {
    package: Package,
    dependencies: HashMap<String, String>,
    bin: Option<Vec<Bin>>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Package {
    name: String,
    version: String,
    edition: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Bin {
    name: String,
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

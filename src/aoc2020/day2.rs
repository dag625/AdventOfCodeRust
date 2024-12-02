use regex::Regex;
use anyhow::{Result, Context};
use crate::utilities;
use std::path::Path;

struct PasswordEntry {
    min: i32,
    max: i32,
    character: char,
    password: String
}

fn parse_password_entry(s: &String) -> anyhow::Result<PasswordEntry> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"^\s*(\d+)-(\d+)\s+(\w):\s+(\w+)\s*$"#).unwrap();
    }
    let found = RE.captures(s).with_context(|| format!("Failed to parse password entry:  {}", s))?;
    Ok(PasswordEntry {
        min: found[1].parse()?,
        max: found[2].parse()?,
        character: found[3].chars().next().context("Failed to get entry's chararacter.")?,
        password: found[4].to_string()
    })
}

fn is_valid_old_job(entry: &PasswordEntry) -> bool {
    let num = entry.password.chars().filter(|c| *c == entry.character).count();
    num >= entry.min as usize && num <= entry.max as usize
}

fn is_valid_new_job(entry: &PasswordEntry) -> bool {
    let c1 = entry.password.chars().nth((entry.min - 1) as usize).unwrap();
    let c2 = entry.password.chars().nth((entry.max - 1) as usize).unwrap();
    (c1 == entry.character && c2 != entry.character) || (c1 != entry.character && c2 == entry.character)
}

fn get_input(input_dir: &Path) -> Result<Vec<PasswordEntry>> {
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_2_input.txt"))?;
    lines.iter().map(|s| parse_password_entry(s).with_context(|| format!("Failed to convert line '{}' to a password entry.", s))).collect()
}

/************************* Part 1 *************************/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let res = get_input(input_dir)?.iter().filter(|e| is_valid_old_job(*e)).count();
    println!("\t{}", res);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let res = get_input(input_dir)?.iter().filter(|e| is_valid_new_job(*e)).count();
    println!("\t{}", res);
    Ok(())
}

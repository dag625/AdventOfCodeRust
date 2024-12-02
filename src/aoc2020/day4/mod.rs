use anyhow::Context;
use std::path::Path;
use anyhow::Result;

mod field_type;

fn get_input(input_dir: &Path) -> Result<Vec<field_type::Id>> {
    let file = std::fs::read_to_string(input_dir.join("2020").join("day_4_input.txt")).context("Failed to read input file for day 4.")?;
    field_type::parse(&file)
}

/************************* Part 1 *************************/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let ids = get_input(input_dir)?;
    let num = ids.iter().filter(|id| id.is_valid(true, false)).count();
    println!("\t{}", num);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let ids = get_input(input_dir)?;
    let num = ids.iter().filter(|id| id.is_valid(true, true)).count();
    println!("\t{}", num);
    Ok(())
}
use anyhow::{Result, Context};
use std::path::Path;
use crate::utilities;

fn get_input(input_file: &Path) -> Result<Vec<i64>> {
    let lines = utilities::get_input_lines(input_file)?;
    lines.iter().map(|s| s.parse().with_context(|| format!("Failed to convert line '{}' to an integer.", s))).collect()
}

const WINDOW: usize = 25;

fn is_valid(data: &[i64]) -> bool {
    if data.len() < WINDOW + 1 {
        return false;
    }
    let val = data[WINDOW];
    for i in 0..WINDOW {
        for j in (i + 1)..WINDOW {
            if data[i] + data[j] == val {
                return true;
            }
        }
    }
    false
}

fn find_first_invalid(data: &Vec<i64>) -> Result<i64> {
    for idx in 0..(data.len() - WINDOW + 1) {
        if !is_valid(&data[idx..idx + WINDOW + 1]) {
            return Ok(data[idx + WINDOW]);
        }
    }
    Err(anyhow::Error::msg("No invalid values found."))
}

fn find_sum_range(data: &Vec<i64>, sum: i64) -> Result<i64> {
    for begin in 0..data.len() {
        for end in (begin + 2)..data.len()
        {
            let s: i64 = data[begin..end].iter().sum();
            if s == sum {
                return Ok(data[begin..end].iter().min().unwrap() + data[begin..end].iter().max().unwrap());
            }
            else if s > sum {
                break;
            }
        }
    }
    Err(anyhow::Error::msg("No matching ranges found."))
}

/************************* Part 1 *************************/
pub fn solve_1(input_file: &Path) -> Result<()> {
    let data = get_input(input_file)?;
    println!("\t{}", find_first_invalid(&data)?);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_file: &Path) -> Result<()> {
    let data = get_input(input_file)?;
    let sum = find_first_invalid(&data)?;
    println!("\t{}", find_sum_range(&data, sum)?);
    Ok(())
}
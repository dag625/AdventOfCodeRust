use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use super::super::utilities;
use anyhow::{Result, Context};

pub fn solve_1(input_file: &Path) -> Result<()> {
    solve(solve_1_impl, input_file)
}

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(&Vec<i32>) -> R, input_file: &Path) -> Result<()> {
    let res = func(&get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_file: &Path) -> Result<Vec<i32>> {
    let lines = utilities::get_input_lines(input_file)?;
    lines.iter().map(|s| s.parse().with_context(|| format!("Failed to convert line '{}' to i32.", s))).collect()
}

/************************* Part 1 *************************/
fn solve_1_impl(input: &Vec<i32>) -> i32 {
    let sum = 2020;
    for i in 0..input.len() {
        for j in (i+1)..input.len() {
            if input[i] + input[j] == sum {
                return input[i] * input[j];
            }
        }
    }
    -1
}

/************************* Part 2 *************************/
fn solve_2_impl(input : &Vec<i32>) -> i64 {
    let sum = 2020;
    for i in 0..input.len() {
        for j in (i + 1)..input.len() {
            for k in (j + 1)..input.len() {
                if input[i] + input[j] + input[k] == sum {
                    return input[i] as i64 * input[j] as i64 * input[k] as i64;
                }
            }
        }
    }
    -1
}
use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use super::super::utilities;
use anyhow::{Result, Context};
use itertools::Itertools;

pub fn solve_1(input_dir: &Path) -> Result<()> {
    solve(solve_1_impl, input_dir)
}

pub fn solve_2(input_dir: &Path) -> Result<()> {
    solve(solve_2_impl, input_dir)
}

fn solve<R: Display>(func: fn(&Vec<i32>) -> R, input_dir: &Path) -> Result<()> {
    let res = func(&get_input(input_dir)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_dir: &Path) -> Result<Vec<i32>> {
    let lines = utilities::get_input_lines(input_dir.join("2021").join("day_1_input.txt"))?;
    lines.iter().map(|s| s.parse().with_context(|| format!("Failed to convert line '{}' to i32.", s))).collect()
}

/************************* Part 1 *************************/
fn solve_1_impl(input: &Vec<i32>) -> i32 {
    let mut count = 0;
    for (a, b) in input.into_iter().tuple_windows() {
        if b > a {
            count += 1;
        }
    }
    count
}

/************************* Part 2 *************************/
fn solve_2_impl(input : &Vec<i32>) -> i64 {
    let mut count = 0;
    for (a, b, c, d) in input.into_iter().tuple_windows() {
        if (b + c + d) > (a + b + c) {
            count += 1;
        }
    }
    count
}
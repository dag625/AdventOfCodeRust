use std::fmt::Display;
use std::path::Path;
use anyhow::{Result, Context};
use crate::utilities;

pub fn solve_1(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day2::solve_1_impl, input_file)
}

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day2::solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(&Vec<Vec<i32>>) -> R, input_file: &Path) -> Result<()> {
    let res = func(&get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_file: &Path) -> Result<Vec<Vec<i32>>> {
    let lines = utilities::get_input_lines(input_file)?;
    Ok(lines.iter().map(|l| l.split(' ')
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i32>().with_context(|| format!("Failed to convert line '{}' to i32.", s)).unwrap())
        .collect()).collect())
}

fn is_safe(report : &[i32]) -> bool {
    (report.is_sorted_by(|a, b| a < b) || report.is_sorted_by(|a, b| b < a)) &&
        report.windows(2).all(|c| {let d = (c[0] - c[1]).abs(); d >= 1 && d <= 3})
}

fn is_safe_removable(report : &[i32]) -> bool {
    for i in 0..report.len() {
        let parts = report.split_at(i);
        let new_rep = [parts.0, &parts.1[1..]].concat();
        if is_safe(&new_rep) {
            return true;
        }
    }
    false
}

pub fn solve_1_impl(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|&l| is_safe(l)).count()
}

pub fn solve_2_impl(input: &Vec<Vec<i32>>) -> usize {
    input.iter().filter(|&l| is_safe_removable(l)).count()
}
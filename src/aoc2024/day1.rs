use std::fmt::Display;
use std::path::Path;
use anyhow::{Result, Context};
use itertools::Itertools;
use crate::utilities;

pub fn solve_1(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day1::solve_1_impl, input_file)
}

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day1::solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(& mut (Vec<i32>, Vec<i32>)) -> R, input_file: &Path) -> Result<()> {
    let res = func(& mut get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_file: &Path) -> Result<(Vec<i32>, Vec<i32>)> {
    let lines = utilities::get_input_lines(input_file)?;
    let pairs : Vec<(i32, i32)> = lines.iter()
        .map(|l| l.split(' ')
            .filter(|&s| !s.is_empty())
            .map(|s| s.parse::<i32>().with_context(|| format!("Failed to convert line '{}' to i32.", s)).unwrap())
            .into_iter().collect_tuple::<(i32, i32)>().unwrap())
        .collect();
    Ok(pairs.into_iter().unzip())
}

pub fn solve_1_impl(input: & mut (Vec<i32>, Vec<i32>)) -> i32 {
    input.0.sort();
    input.1.sort();
    input.0.iter().zip(input.1.iter()).map(|(&x, &y)| (x - y).abs()).sum()
}

pub fn solve_2_impl(input: & mut (Vec<i32>, Vec<i32>)) -> usize {
    input.0.iter().map(|&x| input.1.iter().filter(|&&y| x == y).count() * x as usize).sum()
}
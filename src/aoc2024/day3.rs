use std::fmt::Display;
use std::path::Path;
use anyhow::{Result, Context};
use regex::Regex;
use crate::utilities;

pub fn solve_1(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day3::solve_1_impl, input_file)
}

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day3::solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(&Vec<String>) -> R, input_file: &Path) -> Result<()> {
    let res = func(&get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_file: &Path) -> Result<Vec<String>> {
    utilities::get_input_lines(input_file)
}

fn search_line(line: &String) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(line).map(|c|
        c[1].parse::<i64>().with_context(|| format!("Failed to parse first number in multiplication:  {}", c[0].to_string())).unwrap() *
        c[2].parse::<i64>().with_context(|| format!("Failed to parse second number in multiplication:  {}", c[0].to_string())).unwrap()).sum()
}

fn search_line_toggleable(line: &String, enabled: &mut bool) -> i64 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\))|(do\(\))|(don't\(\))").unwrap();
    re.captures_iter(line).map(|c| {
        let cap_str = c[0].to_string();
        if cap_str.starts_with("don't") {
            *enabled = false;
            0
        }
        else if cap_str.starts_with("do") {
            *enabled = true;
            0
        }
        else if *enabled {
            c[2].parse::<i64>().with_context(|| format!("Failed to parse first number in multiplication:  {}", c[0].to_string())).unwrap() *
                c[3].parse::<i64>().with_context(|| format!("Failed to parse second number in multiplication:  {}", c[0].to_string())).unwrap()
        }
        else {
            0
        }
    }).sum()
}

pub fn solve_1_impl(input: &Vec<String>) -> i64 {
    input.iter().map(search_line).sum()
}

pub fn solve_2_impl(input: &Vec<String>) -> i64 {
    let mut enabled = true;
    input.iter().map(|s| search_line_toggleable(s, & mut enabled)).sum()
}
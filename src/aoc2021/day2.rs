use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use super::super::utilities;
use anyhow::Result;
use crate::aoc2021::day2::Direction::Forward;

pub fn solve_1(input_dir: &Path) -> Result<()> {
    solve(solve_1_impl, input_dir)
}

pub fn solve_2(input_dir: &Path) -> Result<()> {
    solve(solve_2_impl, input_dir)
}

enum Direction {
    Forward(i32),
    Up(i32),
    Down(i32)
}

fn solve<R: Display>(func: fn(&Vec<Direction>) -> R, input_dir: &Path) -> Result<()> {
    let res = func(&get_input(input_dir)?);
    println!("\t{}", res);
    Ok(())
}

fn parse_direction(s: &String) -> Result<Direction> {
    if s.starts_with("forward ") {
        Ok(Forward(s.strip_prefix("forward ").unwrap().parse()?))
    }
    else if s.starts_with("up ") {
        Ok(Direction::Up(s.strip_prefix("up ").unwrap().parse()?))
    }
    else if s.starts_with("down ") {
        Ok(Direction::Down(s.strip_prefix("down ").unwrap().parse()?))
    }
    else {
        Err(anyhow::Error::msg(format!("Failed to parse direction from line '{}'.", s)))
    }
}

fn get_input(input_dir: &Path) -> Result<Vec<Direction>> {
    let lines = utilities::get_input_lines(input_dir.join("2021").join("day_2_input.txt"))?;
    lines.iter().map(|l| parse_direction(l)).collect()
}

/************************* Part 1 *************************/
fn solve_1_impl(input: &Vec<Direction>) -> i32 {
    let mut horiz = 0;
    let mut depth = 0;
    for d in input {
        match d {
            Forward(n) => horiz += n,
            Direction::Up(n) => depth -= n,
            Direction::Down(n) => depth += n

        };
    }
    horiz * depth
}

/************************* Part 2 *************************/
fn solve_2_impl(input : &Vec<Direction>) -> i64 {
    let mut horiz: i64 = 0;
    let mut depth: i64 = 0;
    let mut aim: i64 = 0;
    for d in input {
        match d {
            Forward(n) => { horiz += i64::from(*n); depth += i64::from(*n) * aim; },
            Direction::Up(n) => aim -= i64::from(*n),
            Direction::Down(n) => aim += i64::from(*n)

        };
    }
    horiz * depth
}
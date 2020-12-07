use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use super::super::utilities;
use anyhow::{Result, Context};

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
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_1_input.txt"))?;
    lines.iter().map(|s| s.parse().with_context(|| format!("Failed to convert line '{}' to i32.", s))).collect()
}

/*
Before you leave, the Elves in accounting just need you to fix your expense report (your puzzle input); apparently, something isn't quite adding up.

Specifically, they need you to find the two entries that sum to 2020 and then multiply those two numbers together.

For example, suppose your expense report contained the following:

1721
979
366
299
675
1456
In this list, the two entries that sum to 2020 are 1721 and 299. Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.

Of course, your expense report is much larger. Find the two entries that sum to 2020; what do you get if you multiply them together?
*/
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

/*
The Elves in accounting are thankful for your help; one of them even offers you a starfish coin they had left over from a past vacation. They offer you a second one if you can find three numbers in your expense report that meet the same criteria.

Using the above example again, the three entries that sum to 2020 are 979, 366, and 675. Multiplying them together produces the answer, 241861950.

In your expense report, what is the product of the three entries that sum to 2020?
*/
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
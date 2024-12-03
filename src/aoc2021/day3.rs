use std::path::Path;
use std::fmt::Display;
use std::vec::Vec;
use super::super::utilities;
use anyhow::Result;

pub fn solve_1(input_file: &Path) -> Result<()> { solve(solve_1_impl, input_file) }

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(&Vec<String>) -> R, input_file: &Path) -> Result<()> {
    let res = func(&get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

const INPUT_LEN: usize = 12;

fn find_bit_criteria(data: &Vec<String>, prefix: &String, tie: char, use_ones: fn(usize, usize) -> bool) -> Result<String> {
    let mut one_count = 0;
    let filtered: Vec<&String> = data.iter().filter(|s| {
        let retval = s.starts_with(prefix);
        if retval && prefix.len() < s.len() { one_count += usize::from(s.as_bytes()[prefix.len()] - b'0'); }
        retval
    }).collect();
    let num = filtered.len();
    if num == 1 {
        return Ok(String::from(*filtered.iter().next().unwrap()));
    }

    let mut new_prefix = String::from(prefix);
    if use_ones(one_count, num) {
        new_prefix.push('1');
    }
    else if num % 2 == 0 && one_count == num / 2 {
        new_prefix.push(tie);
    }
    else {
        new_prefix.push('0');
    }

    find_bit_criteria(data, &new_prefix, tie, use_ones)
}

fn get_input(input_file: &Path) -> Result<Vec<String>> {
    utilities::get_input_lines(input_file)
}

/************************* Part 1 *************************/
fn solve_1_impl(input: &Vec<String>) -> u64 {
    let mut counts: [usize; INPUT_LEN] = [0; INPUT_LEN];
    for s in input {
        let mut idx = 0;
        for c in s.as_bytes() {
            counts[idx] += usize::from(c - b'0');
            idx += 1;
        }
    }
    let num = input.len() / 2;
    let mut gamma: u64 = 0;
    let mut mask:u64 = 0;
    for  c in counts {
        gamma <<= 1;
        mask <<= 1;
        gamma |= if c > num { 1 } else { 0 };
        mask |= 1;
    }
    let epsilon = !gamma & mask;
    gamma * epsilon
}

/************************* Part 2 *************************/
fn solve_2_impl(input : &Vec<String>) -> i64 {
    let ox_gen_rating = i64::from_str_radix(
        find_bit_criteria(input, &String::new(), '1',
                              |num_ones, total| num_ones > total / 2)
            .unwrap().as_str(), 2).unwrap();
    let co2_scrub_rating = i64::from_str_radix(
        find_bit_criteria(input, &String::new(), '0',
                              |num_ones, total| num_ones < total / 2 || (total % 2 == 1 && num_ones == total / 2))
            .unwrap().as_str(), 2).unwrap();
    ox_gen_rating * co2_scrub_rating
}
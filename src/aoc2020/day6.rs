use std::path::Path;
use anyhow::Result;
use crate::utilities;

fn count_bits(v: u32) -> u32 {
    let mut retval: u32 = 0;
    for i in 0..26 {
        if (v >> i) & 1 == 1 {
            retval += 1;
        }
    }
    retval
}

fn get_input(input_dir: &Path) -> Result<Vec<Vec<u32>>> {
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_6_input.txt"))?;
    let mut group = Vec::new();
    let mut retval = Vec::new();
    for s in lines {
        if s.is_empty() {
            if !group.is_empty() {
                retval.push(group);
                group = Vec::new();
            }
        }
        else {
            let mut ans : u32 = 0;
            for c in s.chars() {
                ans |= 1 << (c as u32 - 'a' as u32);
            }
            group.push(ans);
        }
    }
    if !group.is_empty() {
        retval.push(group);
    }
    Ok(retval)
}

/************************* Part 1 *************************/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let answers = get_input(input_dir)?;
    let sum: u32 = answers.iter().map(|g| count_bits(g.iter().fold(0u32, |acc, i| acc | i))).sum();
    println!("\t{}", sum);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let answers = get_input(input_dir)?;
    let sum: u32 = answers.iter().map(|g| count_bits(g.iter().fold(0xffffffffu32, |acc, i| acc & i))).sum();
    println!("\t{}", sum);
    Ok(())
}

//------------------------------------------ Tests ------------------------------------------

#[test]
fn test_count_bits() {
    assert_eq!(count_bits(0), 0, "No bits set.");
    assert_eq!(count_bits(1), 1, "One bit set.");
    assert_eq!(count_bits(1 << 24), 1, "One high bit set (24).");
    assert_eq!(count_bits(1 << 25), 1, "One high bit set (25).");
    assert_eq!(count_bits(1 << 26), 0, "One bit set out of range.");
    assert_eq!(count_bits(1 << 25 | 1), 2, "Two bits set.");
    assert_eq!(count_bits(1 << 25 | 1 << 24 | 1 << 14 | 1 << 13 | 1 << 1), 5, "Five bits set.");
    assert_eq!(count_bits(0xffffffff), 26, "Twenty six bits set.");
}
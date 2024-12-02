use crate::utilities;
use anyhow::{Result, Context};
use std::path::Path;
use regex::Regex;

#[derive(Copy, Clone)]
enum Instruction {
    Accumulate{ incr: i32 },
    Jump { offset: i32 },
    Noop { value: i32 }
}

fn parse_instruction(s: &str) -> Result<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(acc|jmp|nop) ([+-]\d+)$").unwrap();
    }
    match RE.captures(s) {
        Some(cap) => {
            match &cap[1] {
                "acc" => Ok(Instruction::Accumulate { incr: cap[2].parse()? }),
                "jmp" => Ok(Instruction::Jump { offset: cap[2].parse()? }),
                "nop" => Ok(Instruction::Noop { value: cap[2].parse()? }),
                _ => Err(anyhow::Error::msg(format!("Unknown instruction '{}'.", &cap[1])))
            }
        },
        None => Err(anyhow::Error::msg(format!("Failed to parse instruction '{}'.", s)))
    }
}

fn get_input(input_dir: &Path) -> Result<Vec<Instruction>> {
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_8_input.txt"))?;
    lines.iter().map(|s| parse_instruction(s).with_context(|| format!("Failed to convert line '{}' to an instruction.", s))).collect()
}

fn safe_increment(index: usize, incr: i32) -> Result<usize> {
    let mut val = index as i32;
    val += incr;
    if val < 0 {
        return Err(anyhow::Error::msg(format!("Requested jump to invalid address {}.", val)));
    }
    Ok(val as usize)
}

fn accumulate_until_loop(instructions: &Vec<Instruction>) -> Result<(i32, bool)> {
    let mut visited: Vec<bool> = instructions.iter().map(|_i| false).collect();
    let mut accumulate = 0;
    let mut index = 0;
    let mut infinite = true;
    loop {
        if index >= instructions.len() {
            infinite = false;
            break;
        }
        if visited[index] {
            break;
        }
        visited[index] = true;
        match instructions[index] {
            Instruction::Noop { value: _value } => { index += 1; },
            Instruction::Accumulate { incr } => { accumulate += incr; index += 1; },
            Instruction::Jump { offset} => { index = safe_increment(index, offset)?; }
        }
    }
    Ok((accumulate, infinite))
}

fn is_acc_cmd(ins: &Instruction) -> bool {
    match ins {
        Instruction::Noop { value: _value } => false,
        Instruction::Jump { offset: _offset } => false,
        Instruction::Accumulate { incr: _incr } => true
    }
}

fn fix_loop(instructions: &Vec<Instruction>) -> Result<i32> {
    for index in 0..instructions.len() {
        if !is_acc_cmd(&instructions[index]) {
            let mut tmp = instructions.to_vec();
            tmp[index] = match instructions[index] {
                Instruction::Accumulate {incr} => Instruction::Accumulate {incr},
                Instruction::Jump {offset} => Instruction::Noop { value: offset },
                Instruction::Noop {value} => Instruction::Jump { offset: value }
            };
            let (acc, infinite) = accumulate_until_loop(&tmp)?;
            if !infinite {
                return Ok(acc);
            }
        }
    }
    Err(anyhow::Error::msg("Failed to fix infinite loop."))
}

/************************* Part 1 *************************/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let instructions = get_input(input_dir)?;
    println!("\t{}", accumulate_until_loop(&instructions)?.0);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let instructions = get_input(input_dir)?;
    println!("\t{}", fix_loop(&instructions)?);
    Ok(())
}
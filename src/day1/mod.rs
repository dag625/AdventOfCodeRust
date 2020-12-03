use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::fmt::Display;
use super::utilities;
use std::result::Result;
use std::vec::Vec;

pub fn solve_day_1_1(input_dir: &Path) -> utilities::Result {
    solve_day_1(solve_day_1_1_impl, input_dir)
}

pub fn solve_day_1_2(input_dir: &Path) -> utilities::Result {
    solve_day_1(solve_day_1_2_impl, input_dir)
}

fn solve_day_1<R: Display>(func: fn(&Vec<i32>) -> R, input_dir: &Path) -> utilities::Result {
    match get_input(input_dir) {
        Ok(vals) => {
            let res = func(&vals);
            println!("\t{}", res);
            Ok(())
        }
        Err(err) => { Err(format!("{}", err)) }
    }
}

fn get_raw_input(input_dir: &Path) -> io::Result<Vec<String>> {
    let mut input_file = PathBuf::from(input_dir);
    input_file.push("day_1_input.txt");
    let f = File::open(input_file)?;
    let rdr = BufReader::new(f);
    let mut retval: Vec<String> = Vec::new();
    for line in rdr.lines() {
        retval.push(line?);
    }
    Ok(retval)
}

fn get_input(input_dir: &Path) -> Result<Vec<i32>, String> {
    let lines = get_raw_input(input_dir);
    match lines {
        Ok(ll) => {
            utilities::list_result_and_convert(ll.iter().map(|l| { l.parse::<i32>() }).collect(),
            |e| format!("{}", e))
        },
        Err(e) => Err(format!("{}", e))
    }
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
fn solve_day_1_1_impl(input: &Vec<i32>) -> i32 {
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
fn solve_day_1_2_impl(input : &Vec<i32>) -> i64 {
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
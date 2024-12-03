use anyhow::{Result, Context};
use crate::utilities;
use std::path::Path;
use std::collections::HashMap;

fn get_input(input_file: &Path) -> Result<Vec<i32>> {
    let lines = utilities::get_input_lines(input_file)?;
    lines.iter().map(|s| s.parse().with_context(|| format!("Failed to convert line '{}' to an integer.", s))).collect()
}

fn count_diffs(sorted: &Vec<i32>) -> (i32, i32) {
    let mut num1s = 0;
    let mut num3s = 0;
    sorted.iter().enumerate().take(sorted.len() - 1).for_each(|(idx, v)| {
        match sorted[idx + 1] - v {
            1 => { num1s = num1s + 1 }
            3 => { num3s = num3s + 1}
            _ => {}
        };
    });
    //We add one to each because there is an implicit adapter at each end at 0 and max+3.
    //We double check that the first link is 1, and if not assume it's 3 (otherwise the
    //puzzle doesn't work).
    let mut del1 = 0;
    let mut del3 = 1;
    if *sorted.first().unwrap() == 1 {
        del1 = del1 + 1;
    }
    else {
        del3 = del3 + 1;
    }
    (num1s + del1, num3s + del3)
}

fn count_paths(sorted: &Vec<i32>) -> i64 {
    let mut map = HashMap::new();
    map.insert(0, 1 as i64);
    for val in sorted {
        let num = map.get(&(val - 1)).unwrap_or(&(0)) +
            map.get(&(val - 2)).unwrap_or(&(0)) +
            map.get(&(val - 3)).unwrap_or(&(0));
        map.insert(*val, num);
    }
    *map.get(sorted.last().unwrap()).unwrap()
}

/************************* Part 1 *************************/
pub fn solve_1(input_file: &Path) -> Result<()> {
    let mut data = get_input(input_file)?;
    data.sort();
    let (sum1s, sum3s) = count_diffs(&data);
    println!("\t{}", sum1s * sum3s);
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_file: &Path) -> Result<()> {
    let mut data = get_input(input_file)?;
    data.sort();
    println!("\t{}", count_paths(&data));
    Ok(())
}

//------------------------------------------ Tests ------------------------------------------

#[test]
fn test_example_input() {
    let mut data = Vec::from([
            28,
            33,
            18,
            42,
            31,
            14,
            46,
            20,
            48,
            47,
            24,
            23,
            49,
            45,
            19,
            38,
            39,
            11,
            1,
            32,
            25,
            35,
            8,
            17,
            7,
            9,
            4,
            2,
            34,
            10,
            3
    ]);
    data.sort();
    assert_eq!(count_diffs(&data), (22,10), "Mismatch with challenge example.");
    assert_eq!(count_paths(&data), 19208, "Mismatch counting possible paths with challenge example.");
}

#[test]
fn test_simple_example_input() {
    let mut data = Vec::from([
            16,
            10,
            15,
            5,
            1,
            11,
            7,
            19,
            6,
            12,
            4
    ]);
    data.sort();
    assert_eq!(count_diffs(&data), (7,5), "Mismatch with challenge example.");
    assert_eq!(count_paths(&data), 8, "Mismatch counting possible paths with challenge example.");
}
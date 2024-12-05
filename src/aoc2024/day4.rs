//
// Created on 12/4/2024.
//

use std::fmt::Display;
use std::path::Path;
use anyhow::Result;
use itertools::Itertools;
use ndarray::{s, Array1, Array2, ArrayView1, ArrayView2, Axis};
use crate::utilities;

pub fn solve_1(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day4::solve_1_impl, input_file)
}

pub fn solve_2(input_file: &Path) -> Result<()> {
    solve(crate::aoc2024::day4::solve_2_impl, input_file)
}

fn solve<R: Display>(func: fn(&Array2<char>) -> R, input_file: &Path) -> Result<()> {
    let res = func(&get_input(input_file)?);
    println!("\t{}", res);
    Ok(())
}

fn get_input(input_file: &Path) -> Result<Array2<char>> {
    let lines = utilities::get_input_lines(input_file)?;
    let mut retval = Array2::<char>::default((lines.len(), lines.first().unwrap().len()));
    for (row_idx, mut row) in retval.axis_iter_mut(Axis(0)).enumerate() {
        for (col_idx, col) in row.iter_mut().enumerate() {
            *col = lines[row_idx].chars().nth(col_idx).unwrap();
        }
    }
    Ok(retval)
}

fn check_for_str(view: &ArrayView1<char>, to_find: &str) -> i32 {
    let mut retval = 0;
    for i in 0..(view.len() - to_find.len() + 1) {
        if view.iter().skip(i).take(to_find.len()).zip_eq(to_find.chars()).all(|(&a, b)| a == b) ||
        view.iter().skip(i).take(to_find.len()).rev().zip_eq(to_find.chars()).all(|(&a, b)| a == b)
        {
            retval += 1;
        }
    }
    retval
}

fn check_for_str_in_diag(view: &ArrayView2<char>, to_find: &str) -> i32 {
    assert!(view.nrows() == to_find.len());
    assert!(view.ncols() == to_find.len());
    let mut retval = 0;
    let mut forward = Array1::<char>::default(to_find.len());
    let mut backward = Array1::<char>::default(to_find.len());
    for i in 0..to_find.len() {
        forward[i] = *view.get((i, i)).unwrap();
        backward[i] = *view.get((i, to_find.len() - 1 - i)).unwrap();
    }
    if forward.iter().zip_eq(to_find.chars()).all(|(&a,b)| a == b) ||
        forward.iter().zip_eq(to_find.chars().rev()).all(|(&a,b)| a == b)
    {
        retval += 1;
    }
    if backward.iter().zip_eq(to_find.chars()).all(|(&a,b)| a == b) ||
        backward.iter().zip_eq(to_find.chars().rev()).all(|(&a,b)| a == b)
    {
        retval += 1;
    }
    retval
}

fn check_for_x_mas(view: &ArrayView2<char>) -> bool {
    if *view.get((1,1)).unwrap() == 'A' {
        return (*view.get((0,0)).unwrap() == 'M' && *view.get((0,2)).unwrap() == 'M' && *view.get((2,2)).unwrap() == 'S' && *view.get((2,0)).unwrap() == 'S') ||
            (*view.get((0,0)).unwrap() == 'S' && *view.get((0,2)).unwrap() == 'M' && *view.get((2,2)).unwrap() == 'M' && *view.get((2,0)).unwrap() == 'S') ||
            (*view.get((0,0)).unwrap() == 'S' && *view.get((0,2)).unwrap() == 'S' && *view.get((2,2)).unwrap() == 'M' && *view.get((2,0)).unwrap() == 'M') ||
            (*view.get((0,0)).unwrap() == 'M' && *view.get((0,2)).unwrap() == 'S' && *view.get((2,2)).unwrap() == 'S' && *view.get((2,0)).unwrap() == 'M');
    }
    false
}

pub fn solve_1_impl(input: &Array2<char>) -> i32 {
    const XMAS : &str = "XMAS";
    let mut retval = 0;
    for r in 0..(input.nrows() - XMAS.len() + 1) {
        for c in 0..(input.ncols() - XMAS.len() + 1) {
            let sub = input.slice(s![r..(r+XMAS.len()), c..(c+XMAS.len())]);
            retval += check_for_str_in_diag(&sub, XMAS);
        }
    }
    retval += input.axis_iter(Axis(0)).map(|r| check_for_str(&r, XMAS)).sum::<i32>();
    retval += input.axis_iter(Axis(1)).map(|c| check_for_str(&c, XMAS)).sum::<i32>();
    retval
}

pub fn solve_2_impl(input: &Array2<char>) -> i32 {
    let mut retval = 0;
    for r in 0..(input.nrows() - 2) {
        for c in 0..(input.ncols() - 2) {
            let sub = input.slice(s![r..(r+3), c..(c+3)]);
            if check_for_x_mas(&sub) {
                retval += 1;
            }
        }
    }
    retval
}
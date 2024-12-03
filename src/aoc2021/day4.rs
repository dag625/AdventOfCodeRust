// use std::path::Path;
// use std::fmt::Display;
// use std::vec::Vec;
// use super::super::utilities;
// use anyhow::Result;
// use ndarray::arr2;
//use Square::Uncalled;
//use Square::Called;
//
// pub fn solve_1(input_file: &Path) -> Result<()> {
//     solve(solve_1_impl, input_file)
// }
//
// pub fn solve_2(input_file: &Path) -> Result<()> {
//     solve(solve_2_impl, input_file)
// }
//
// fn solve<R: Display>(func: fn((Vec<u8>, Vec<Board>)) -> R, input_file: &Path) -> Result<()> {
//     let res = func(get_input(input_file)?);
//     println!("\t{}", res);
//     Ok(())
// }
//
// enum Square {
//     Uncalled(u8),
//     Called(u8)
// }
//
// impl Clone for Square {
//     fn clone(&self) -> Self {
//         match self {
//             Uncalled(x) => Uncalled(*x),
//             Called(x) => Square::Called(*x)
//         }
//     }
//
//     fn clone_from(&mut self, source: &Self) {
//         match source {
//             Uncalled(x) => *self = Uncalled(*x),
//             Called(x) => *self = Called(*x)
//         }
//     }
// }
//
// impl Copy for Square {}
//
// struct Board {
//     squares: [[Square; 5]; 5]
// }
//
// fn check_bingo(arr: &mut [Square], val: u8) -> bool {
//     let mut is_bingo = true;
//     for n in arr {
//         match n {
//             Uncalled(num) => { if *num == val { *n = Called(*num); } else { is_bingo = false; } }
//             Called(_) => {}
//         }
//     }
//     is_bingo
// }
//
// impl Board {
//     pub fn have_bingo(&mut self, val: u8) -> bool {
//         for r in &mut self.squares {
//             if check_bingo(r.as_mut(), val) {
//                 return true;
//             }
//         }
//         for c in 0..5 {
//             if check_bingo(arr2(&self.squares).column_mut(c).as_slice_mut().unwrap(), val) {
//                 return true;
//             }
//         }
//         false
//     }
//
//     pub fn sum_unmarked(&self) -> i64 {
//         self.squares.iter().flatten().into_iter().fold(0, |v, s| match *s {
//             Uncalled(n) => { v + i64::from(n) }
//             Called(_) => { v }
//         })
//     }
// }
//
// fn get_input(input_file: &Path) -> Result<(Vec<u8>, Vec<Board>)> {
//     let lines = utilities::get_input_lines(input_file)?;
//     let numbers = lines[0].split(',').into_iter().map(|s| s.parse().unwrap()).collect();
//     let mut boards: Vec<Board> = Vec::new();
//     let mut row: usize = 0;
//     for l in lines.iter().skip(1) {
//         if l.is_empty() {
//             boards.push(Board{squares: [[Uncalled(0); 5]; 5]});
//             row = 0;
//         }
//         else {
//             let idx = boards.len() - 1;
//             let &mut mut dest = &mut boards[idx].squares[row];
//             let vals: Vec<u8> = l.split_whitespace().map(|s| s.parse().unwrap()).collect();
//             for idx in 0..5 {
//                 dest[idx] = Uncalled(vals[idx]);
//             }
//             row += 1;
//         }
//     }
//
//     Ok((numbers, boards))
// }
//
// /*
//
// */
// fn solve_1_impl(mut input: (Vec<u8>, Vec<Board>)) -> i64 {
//     // for val in input.0 {
//     //     for b in &mut input.1 {
//     //         if b.have_bingo(val) {
//     //             return i64::from(val) * b.sum_unmarked();
//     //         }
//     //     }
//     // }
//     0
// }
//
// /*
//
// */
// fn solve_2_impl(mut input : (Vec<u8>, Vec<Board>)) -> i64 {
//     0
// }
use anyhow::{Result, Context};
use crate::utilities;
use std::path::Path;

struct Seat {
    row: i8,
    col: i8
}

impl Seat {
    pub fn seat_id(&self) -> i32 {
        self.row as i32 * 8 + self.col as i32
    }
}

fn parse_seat(s: &String) -> Result<Seat> {
    if s.len() != 10 {
        return Err(anyhow::Error::msg(format!("Boarding pass has incorrect length {}.", s.len())));
    }
    let row_str = &s[..7];
    let col_str = &s[7..];

    const ROW_DENOM: i8 = 'B' as i8 - 'F' as i8;
    let r = row_str.chars().fold(0i8, |mut acc, ch| { acc <<= 1; acc | ((ch as i8 - 'F' as i8) / ROW_DENOM) });
    const COL_DENOM: i8 = 'R' as i8 - 'L' as i8;
    let c = col_str.chars().fold(0i8, |mut acc, ch| { acc <<= 1; acc | ((ch as i8 - 'L' as i8) / COL_DENOM) });
    Ok(Seat{ row: r, col: c})
}

fn get_input(input_file: &Path) -> Result<Vec<Seat>> {
    let lines = utilities::get_input_lines(input_file)?;
    let retval : Result<Vec<Seat>> = lines.iter().map(|s| parse_seat(s).with_context(|| format!("Failed to convert line '{}' to a seat.", s))).collect();
    let mut retval = retval?;
    retval.sort_by(|a, b| a.seat_id().cmp(&b.seat_id()));
    Ok(retval)
}

/************************* Part 1 *************************/
pub fn solve_1(input_file: &Path) -> Result<()> {
    let seats = get_input(input_file)?;
    println!("\t{}", seats.last().ok_or(anyhow::Error::msg("No seats in list."))?.seat_id());
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_file: &Path) -> Result<()> {
    let seats = get_input(input_file)?;
    let mut next_id = seats.first().ok_or(anyhow::Error::msg("Empty list of seats."))?.seat_id();
    for s in seats {
        if next_id != s.seat_id() {
            break;
        }
        next_id += 1;
    }
    println!("\t{}", next_id);
    Ok(())
}
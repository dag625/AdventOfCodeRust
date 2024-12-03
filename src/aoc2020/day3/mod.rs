mod velocity;
mod position;

use std::path::Path;
use anyhow::{Result, Context};
use crate::utilities;
use position::Position;
use velocity::Velocity;

const OPEN_SPACE : char = '.';
const TREE_SPACE : char = '#';
const VALID_VALUES : [char; 2] = [OPEN_SPACE, TREE_SPACE];

type Map = Vec<Vec<char>>;

fn get_input(input_file: &Path) -> Result<Map> {
    let lines : Map = utilities::get_input_lines(input_file)?
        .iter().map(|s| s.chars().collect()).collect();
    let line_len = lines.first().context("Cannot generate map without any lines.")?.len();
    if !lines.iter().all(|l| l.len() == line_len && l.iter().all(|c| VALID_VALUES.contains(c))) {
        return Err(anyhow::Error::msg("Invalid input data.  Input must be a non-empty \
                set of non-empty lines of the same length containing \
                only '.' and '#' characters."));
    }
    Ok(lines)
}

fn trees_in_space(map: &Map, p: Position) -> i32 {
    //We assume wrap() has been called on the position since the last change.
    const DENOM: i32 = TREE_SPACE as i32 - OPEN_SPACE as i32;
    (map[p.y as usize][p.x as usize] as i32 - OPEN_SPACE as i32) / DENOM
}

fn num_trees_in_path(map: &Map, mut pos: Position, vel: Velocity) -> i32 {
    let mut num_trees = 0;
    while pos.wrap(map) {
        num_trees += trees_in_space(map, pos);
        pos += vel;
    }
    num_trees
}

/************************* Part 1 *************************/
pub fn solve_1(input_file: &Path) -> Result<()> {
    let map = get_input(input_file)?;
    println!("\t{}", num_trees_in_path(&map, Position::top_left(), Velocity{ dx: 3, dy: 1 }));
    Ok(())
}

/************************* Part 2 *************************/
pub fn solve_2(input_file: &Path) -> Result<()> {
    let map = get_input(input_file)?;
    let pos = Position::top_left();
    let vels = [
        Velocity{ dx:1, dy: 1 },
        Velocity{ dx:3, dy: 1 },
        Velocity{ dx:5, dy: 1 },
        Velocity{ dx:7, dy: 1 },
        Velocity{ dx:1, dy: 2 }
    ];
    let res = vels.iter().map(|v| num_trees_in_path(&map, pos, *v) as i64).fold(1, |acc, it| acc * it);
    println!("\t{}", res);
    Ok(())
}
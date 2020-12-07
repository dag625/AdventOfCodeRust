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

fn get_input(input_dir: &Path) -> Result<Map> {
    let lines : Map = utilities::get_input_lines(input_dir.join("2020").join("day_3_input.txt"))?
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

/*
With the toboggan login problems resolved, you set off toward the airport. While travel by toboggan might be easy, it's certainly not safe: there's very minimal steering and the area is covered in trees. You'll need to see which angles will take you near the fewest trees.

Due to the local geology, trees in this area only grow on exact integer coordinates in a grid. You make a map (your puzzle input) of the open squares (.) and trees (#) you can see. For example:

..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
These aren't the only trees, though; due to something you read about once involving arboreal genetics and biome stability, the same pattern repeats to the right many times:

..##.........##.........##.........##.........##.........##.......  --->
#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.##.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........#.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...##....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
You start on the open square (.) in the top-left corner and need to reach the bottom (below the bottom-most row on your map).

The toboggan can only follow a few specific slopes (you opted for a cheaper model that prefers rational numbers); start by counting all the trees you would encounter for the slope right 3, down 1:

From your starting position at the top-left, check the position that is right 3 and down 1. Then, check the position that is right 3 and down 1 from there, and so on until you go past the bottom of the map.

The locations you'd check in the above example are marked here with O where there was an open square and X where there was a tree:

..##.........##.........##.........##.........##.........##.......  --->
#..O#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#..
.#....X..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#.
..#.#...#O#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#
.#...##..#..X...##..#..#...##..#..#...##..#..#...##..#..#...##..#.
..#.##.......#.X#.......#.##.......#.##.......#.##.......#.##.....  --->
.#.#.#....#.#.#.#.O..#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#
.#........#.#........X.#........#.#........#.#........#.#........#
#.##...#...#.##...#...#.X#...#...#.##...#...#.##...#...#.##...#...
#...##....##...##....##...#X....##...##....##...##....##...##....#
.#..#...#.#.#..#...#.#.#..#...X.#.#..#...#.#.#..#...#.#.#..#...#.#  --->
In this example, traversing the map using this slope would cause you to encounter 7 trees.

Starting at the top-left corner of your map and following a slope of right 3 and down 1, how many trees would you encounter?
*/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let map = get_input(input_dir)?;
    println!("\t{}", num_trees_in_path(&map, Position::top_left(), Velocity{ dx: 3, dy: 1 }));
    Ok(())
}

/*
Time to check the rest of the slopes - you need to minimize the probability of a sudden arboreal stop, after all.

Determine the number of trees you would encounter if, for each of the following slopes, you start at the top-left corner and traverse the map all the way to the bottom:

Right 1, down 1.
Right 3, down 1. (This is the slope you already checked.)
Right 5, down 1.
Right 7, down 1.
Right 1, down 2.
In the above example, these slopes would find 2, 7, 3, 4, and 2 tree(s) respectively; multiplied together, these produce the answer 336.

What do you get if you multiply together the number of trees encountered on each of the listed slopes?
*/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let map = get_input(input_dir)?;
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
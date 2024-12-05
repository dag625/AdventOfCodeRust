#[macro_use] extern crate lazy_static;

mod aoc2020;
mod aoc2021;
mod utilities;
mod aoc2024;

use std::env;
use std::time::Instant;

type ChallengeFunction = fn(&std::path::Path) -> anyhow::Result<()>;

struct Challenge {
    year: i32,
    day: i32,
    num: i32,
    function: ChallengeFunction
}

impl Challenge {
    pub fn matches(&self, ryear: i32, rday: i32, rnum: i32) -> bool {
        if ryear <= 0 {
            true
        }
        else if ryear == self.year {
            if rday <= 0 {
                true
            } else if rday == self.day {
                if rnum <= 0 || rnum == self.num {
                    true
                } else {
                    false
                }
            } else {
                false
            }
        }
        else {
            false
        }
    }

    pub fn run(&self, input_dir: &std::path::Path) {
        let start = Instant::now();
        println!("{}", self.to_string());
        match (self.function)(input_dir.join(self.year.to_string()).join(format!("day_{}_input.txt", self.day)).as_path()) {
            Ok(()) => {},
            Err(err) => { println!("\t{} failed due to an error:  {}", self.to_string(), err); }
        };
        let duration = start.elapsed();
        println!("\tChallenge time:  {:?}", duration);
    }

    pub fn to_string(&self) -> String {
        format!("Year {} - Day {} - Challenge {}", self.year, self.day, self.num)
    }
}

const CHALLENGES: &[Challenge] = &[
    //2020:
    Challenge{ year: 2020, day: 1, num: 1, function: aoc2020::day1::solve_1},
    Challenge{ year: 2020, day: 1, num: 2, function: aoc2020::day1::solve_2},
    Challenge{ year: 2020, day: 2, num: 1, function: aoc2020::day2::solve_1},
    Challenge{ year: 2020, day: 2, num: 2, function: aoc2020::day2::solve_2},
    Challenge{ year: 2020, day: 3, num: 1, function: aoc2020::day3::solve_1},
    Challenge{ year: 2020, day: 3, num: 2, function: aoc2020::day3::solve_2},
    Challenge{ year: 2020, day: 4, num: 1, function: aoc2020::day4::solve_1},
    Challenge{ year: 2020, day: 4, num: 2, function: aoc2020::day4::solve_2},
    Challenge{ year: 2020, day: 5, num: 1, function: aoc2020::day5::solve_1},
    Challenge{ year: 2020, day: 5, num: 2, function: aoc2020::day5::solve_2},
    Challenge{ year: 2020, day: 6, num: 1, function: aoc2020::day6::solve_1},
    Challenge{ year: 2020, day: 6, num: 2, function: aoc2020::day6::solve_2},
    Challenge{ year: 2020, day: 7, num: 1, function: aoc2020::day7::solve_1},
    Challenge{ year: 2020, day: 7, num: 2, function: aoc2020::day7::solve_2},
    Challenge{ year: 2020, day: 8, num: 1, function: aoc2020::day8::solve_1},
    Challenge{ year: 2020, day: 8, num: 2, function: aoc2020::day8::solve_2},
    Challenge{ year: 2020, day: 9, num: 1, function: aoc2020::day9::solve_1},
    Challenge{ year: 2020, day: 9, num: 2, function: aoc2020::day9::solve_2},
    Challenge{ year: 2020, day: 10, num: 1, function: aoc2020::day10::solve_1},
    Challenge{ year: 2020, day: 10, num: 2, function: aoc2020::day10::solve_2},
    //2021:
    Challenge{ year: 2021, day: 1, num: 1, function: aoc2021::day1::solve_1},
    Challenge{ year: 2021, day: 1, num: 2, function: aoc2021::day1::solve_2},
    Challenge{ year: 2021, day: 2, num: 1, function: aoc2021::day2::solve_1},
    Challenge{ year: 2021, day: 2, num: 2, function: aoc2021::day2::solve_2},
    Challenge{ year: 2021, day: 3, num: 1, function: aoc2021::day3::solve_1},
    Challenge{ year: 2021, day: 3, num: 2, function: aoc2021::day3::solve_2},
    //Challenge{ year: 2021, day: 4, num: 1, function: aoc2021::day4::solve_1},
    //Challenge{ year: 2021, day: 4, num: 2, function: aoc2021::day4::solve_2},
    //2024:
    Challenge{ year: 2024, day: 1, num: 1, function: aoc2024::day1::solve_1},
    Challenge{ year: 2024, day: 1, num: 2, function: aoc2024::day1::solve_2},
    Challenge{ year: 2024, day: 2, num: 1, function: aoc2024::day2::solve_1},
    Challenge{ year: 2024, day: 2, num: 2, function: aoc2024::day2::solve_2},
    Challenge{ year: 2024, day: 3, num: 1, function: aoc2024::day3::solve_1},
    Challenge{ year: 2024, day: 3, num: 2, function: aoc2024::day3::solve_2},
    Challenge{ year: 2024, day: 4, num: 1, function: aoc2024::day4::solve_1},
    Challenge{ year: 2024, day: 4, num: 2, function: aoc2024::day4::solve_2}
];

fn main_impl() -> std::result::Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err("Need directory containing input files.".to_string());
    }
    let input_dir = std::path::Path::new(&args[1]);
    if !input_dir.exists() {
        return Err(format!("Input directory '{}' does not exist.", input_dir.to_str().unwrap_or_default()));
    }
    else if !input_dir.is_dir() {
        return Err(format!("Input directory '{}' is not a directory.", input_dir.to_str().unwrap_or_default()));
    }

    //There's a better way to protect against the "--profile dev" arguments added by RustRover, but I don't care right now.
    let mut year = 0;
    if args.len() > 2 && !args[2].starts_with("--") {
        year = args[2].parse::<i32>().map_err(|e| format!("{}", e))?;
    }
    let mut day = 0;
    if args.len() > 3 && !args[2].starts_with("--") && !args[3].starts_with("--") {
        day = args[3].parse::<i32>().map_err(|e| format!("{}", e))?;
    }
    let mut chal = 0;
    if args.len() > 4 && !args[2].starts_with("--") && !args[3].starts_with("--") && !args[4].starts_with("--") {
        chal = args[4].parse::<i32>().map_err(|e| format!("{}", e))?;
    }

    let start = Instant::now();
    CHALLENGES.iter().filter(|c| c.matches(year, day, chal)).map(|c| c.run(input_dir)).for_each(|_c| {});
    let duration = start.elapsed();

    println!("Finished solutions in:  {:?}", duration);

    Ok(())
}

fn main() {
    match main_impl() {
        Ok(()) => { std::process::exit(0); },
        Err(e) => {
            println!("Failed due to error:  {}", e);
            std::process::exit(1);
        }
    }
}

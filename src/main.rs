mod day1;
mod utilities;

use std::env;

type ChallengeFunction = fn(&std::path::Path) -> utilities::Result;

struct Challenge {
    day: i32,
    num: i32,
    function: ChallengeFunction
}

impl Challenge {
    pub fn matches(&self, rday: i32, rnum: i32) -> bool {
        if rday <= 0 {
            true
        }
        else if rday == self.day {
            if rnum <= 0 || rnum == self.num {
                true
            }
            else {
                false
            }
        }
        else {
            false
        }
    }

    pub fn run(&self, input_dir: &std::path::Path) {
        println!("{}", self.to_string());
        match (self.function)(input_dir) {
            Ok(()) => {},
            Err(err) => { println!("\t{} failed due to an error:  {}", self.to_string(), err); }
        }
    }

    pub fn to_string(&self) -> String {
        format!("Day {} - Challenge {}", self.day, self.num)
    }
}

const CHALLENGES: &[Challenge] = &[
    Challenge{ day: 1, num: 1, function: day1::solve_day_1_1},
    Challenge{ day: 1, num: 2, function: day1::solve_day_1_2}
];

fn main_impl() -> utilities::Result {
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

    let mut day = 0;
    if args.len() > 2 {
        day = args[2].parse::<i32>().map_err(|e| format!("{}", e))?;
    }
    let mut chal = 0;
    if args.len() > 3 {
        chal = args[3].parse::<i32>().map_err(|e| format!("{}", e))?;
    }

    CHALLENGES.iter().filter(|c| c.matches(day, chal)).map(|c| c.run(input_dir)).for_each(|_c| {});

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

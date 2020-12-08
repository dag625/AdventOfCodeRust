use anyhow::{Result, Context};
use regex::Regex;
use std::path::Path;
use crate::utilities;

struct BagRequirement {
    name: String,
    num: i32
}

struct BagType {
    name: String,
    reqs: Vec<BagRequirement>
}

fn parse_bag_type(s: &str) -> Result<BagType> {
    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^([a-z]+\s[a-z]+)\sbags\scontain\s").unwrap();
        static ref RE2: Regex = Regex::new(r"(\d+)\s([a-z]+\s[a-z]+)\sbag[s]?[,.]").unwrap();
    }

    let n = match RE1.captures(s) {
        Some(cap) => { cap[1].to_string() }
        None => { return Err(anyhow::Error::msg("")); }
    };

    let mut req_list = Vec::new();
    for cap in RE2.captures_iter(s) {
        req_list.push(BagRequirement{num: cap[1].parse()?, name: cap[2].to_string()});
    }
    Ok(BagType{name: n, reqs: req_list})
}

fn get_input(input_dir: &Path) -> Result<Vec<BagType>> {
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_7_input.txt"))?;
    lines.iter().map(|s| parse_bag_type(s).with_context(|| format!("Failed to convert line '{}' to a bag type description.", s))).collect()
}

fn get_parents<'a, 'b>(bags: &'a Vec<BagType>, child: &'b str) -> Vec<&'a str> {
    bags.iter().filter(|b| b.reqs.iter().any(|r| r.name == child)).map(|b| b.name.as_str()).collect()
}

fn get_root_ancestors<'a, 'b>(bags: &'a Vec<BagType>, child: &'b str) -> Vec<&'a str> {
    let mut retval: Vec<&'a str> = get_parents(bags, child).iter().flat_map(|b| {let mut gp = get_root_ancestors(bags,b); gp.push(b); gp}).collect();
    retval.sort();
    retval.dedup();
    retval
}

fn get_bag_info<'a, 'b>(bags: &'a Vec<BagType>, name: &'b str) -> Result<&'a BagType> {
    match bags.iter().filter(|b| b.name == name).take(1).last() {
        Some(b) => Ok(b),
        None => Err(anyhow::Error::msg(format!("No bag information found for '{}'.", name)))
    }
}

fn count_descendants(bags: &Vec<BagType>, name: &str) -> Result<i64> {
    get_bag_info(bags, name)?.reqs.iter().map(|b| count_descendants(bags, b.name.as_str()).map(|v| (v + 1) * b.num as i64)).sum()
}

/*
You land at the regional airport in time for your next flight. In fact, it looks like you'll even have time to grab some food: all flights are currently delayed due to issues in luggage processing.

Due to recent aviation regulations, many rules (your puzzle input) are being enforced about bags and their contents; bags must be color-coded and must contain specific quantities of other color-coded bags. Apparently, nobody responsible for these regulations considered how long they would take to enforce!

For example, consider the following rules:

light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.
These rules specify the required contents for 9 bag types. In this example, every faded blue bag is empty, every vibrant plum bag contains 11 bags (5 faded blue and 6 dotted black), and so on.

You have a shiny gold bag. If you wanted to carry it in at least one other bag, how many different bag colors would be valid for the outermost bag? (In other words: how many colors can, eventually, contain at least one shiny gold bag?)

In the above rules, the following options would be available to you:

A bright white bag, which can hold your shiny gold bag directly.
A muted yellow bag, which can hold your shiny gold bag directly, plus some other bags.
A dark orange bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
A light red bag, which can hold bright white and muted yellow bags, either of which could then hold your shiny gold bag.
So, in this example, the number of bag colors that can eventually contain at least one shiny gold bag is 4.

How many bag colors can eventually contain at least one shiny gold bag? (The list of rules is quite long; make sure you get all of it.)
*/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let bags = get_input(input_dir)?;
    println!("\t{}", get_root_ancestors(&bags, "shiny gold").len());
    Ok(())
}

/*
It's getting pretty expensive to fly these days - not because of ticket prices, but because of the ridiculous number of bags you need to buy!

Consider again your shiny gold bag and the rules from the above example:

faded blue bags contain 0 other bags.
dotted black bags contain 0 other bags.
vibrant plum bags contain 11 other bags: 5 faded blue bags and 6 dotted black bags.
dark olive bags contain 7 other bags: 3 faded blue bags and 4 dotted black bags.
So, a single shiny gold bag must contain 1 dark olive bag (and the 7 bags within it) plus 2 vibrant plum bags (and the 11 bags within each of those): 1 + 1*7 + 2 + 2*11 = 32 bags!

Of course, the actual rules have a small chance of going several levels deeper than this example; be sure to count all of the bags, even if the nesting becomes topologically impractical!

Here's another example:

shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.
In this example, a single shiny gold bag must contain 126 other bags.

How many individual bags are required inside your single shiny gold bag?
*/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let bags = get_input(input_dir)?;
    println!("\t{}", count_descendants(&bags, "shiny gold")?);
    Ok(())
}

//------------------------------------------ Tests ------------------------------------------

#[test]
fn test_get_ancestors() {
    let data = Vec::from([
        BagType{ name: "a".to_string(), reqs: Vec::from([ BagRequirement{ name: "b".to_string(), num: 1 }, BagRequirement{ name: "c".to_string(), num: 1 } ]) },
        BagType{ name: "b".to_string(), reqs: Vec::from([ BagRequirement{ name: "e".to_string(), num: 1 } ]) },
        BagType{ name: "c".to_string(), reqs: Vec::from([ BagRequirement{ name: "b".to_string(), num: 1 }, BagRequirement{ name: "e".to_string(), num: 1 } ]) },
        BagType{ name: "d".to_string(), reqs: Vec::from([ BagRequirement{ name: "a".to_string(), num: 1 }, BagRequirement{ name: "c".to_string(), num: 1 } ]) },
        BagType{ name: "e".to_string(), reqs: Vec::from([  ]) }
    ]);
    assert_eq!(get_root_ancestors(&data, "a").len(), 1, "Count of 'a'.");
    assert_eq!(get_root_ancestors(&data, "b").len(), 3, "Count of 'b'.");
    assert_eq!(get_root_ancestors(&data, "c").len(), 2, "Count of 'c'.");
    assert_eq!(get_root_ancestors(&data, "d").len(), 0, "Count of 'd'.");
    assert_eq!(get_root_ancestors(&data, "e").len(), 4, "Count of 'e'.");
}

#[test]
fn test_part_one() {
    let data = Vec::from([
        parse_bag_type("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap(),
        parse_bag_type("dark orange bags contain 3 bright white bags, 4 muted yellow bags.").unwrap(),
        parse_bag_type("bright white bags contain 1 shiny gold bag.").unwrap(),
        parse_bag_type("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.").unwrap(),
        parse_bag_type("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.").unwrap(),
        parse_bag_type("dark olive bags contain 3 faded blue bags, 4 dotted black bags.").unwrap(),
        parse_bag_type("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.").unwrap(),
        parse_bag_type("faded blue bags contain no other bags.").unwrap(),
        parse_bag_type("dotted black bags contain no other bags.").unwrap()
    ]);
    assert_eq!(data[0].reqs.len(), 2, "1st rule:  Number of bag types.");
    assert_eq!(data[0].reqs.iter().map(|r| r.num).sum::<i32>(), 3, "1st rule:  Number of bags.");
}

#[test]
fn test_part_two() {
    let data = Vec::from([
        parse_bag_type("light red bags contain 1 bright white bag, 2 muted yellow bags.").unwrap(),
        parse_bag_type("dark orange bags contain 3 bright white bags, 4 muted yellow bags.").unwrap(),
        parse_bag_type("bright white bags contain 1 shiny gold bag.").unwrap(),
        parse_bag_type("muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.").unwrap(),
        parse_bag_type("shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.").unwrap(),
        parse_bag_type("dark olive bags contain 3 faded blue bags, 4 dotted black bags.").unwrap(),
        parse_bag_type("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.").unwrap(),
        parse_bag_type("faded blue bags contain no other bags.").unwrap(),
        parse_bag_type("dotted black bags contain no other bags.").unwrap()
    ]);
    assert_eq!(count_descendants(&data, "shiny gold").unwrap(), 32)
}
use std::path::Path;
use anyhow::Result;
use crate::utilities;

fn count_bits(v: u32) -> u32 {
    let mut retval: u32 = 0;
    for i in 0..26 {
        if (v >> i) & 1 == 1 {
            retval += 1;
        }
    }
    retval
}

fn get_input(input_dir: &Path) -> Result<Vec<Vec<u32>>> {
    let lines = utilities::get_input_lines(input_dir.join("2020").join("day_6_input.txt"))?;
    let mut group = Vec::new();
    let mut retval = Vec::new();
    for s in lines {
        if s.is_empty() {
            if !group.is_empty() {
                retval.push(group);
                group = Vec::new();
            }
        }
        else {
            let mut ans : u32 = 0;
            for c in s.chars() {
                ans |= 1 << (c as u32 - 'a' as u32);
            }
            group.push(ans);
        }
    }
    if !group.is_empty() {
        retval.push(group);
    }
    Ok(retval)
}

/*
As your flight approaches the regional airport where you'll switch to a much larger plane, customs declaration forms are distributed to the passengers.

The form asks a series of 26 yes-or-no questions marked a through z. All you need to do is identify the questions for which anyone in your group answers "yes". Since your group is just you, this doesn't take very long.

However, the person sitting next to you seems to be experiencing a language barrier and asks if you can help. For each of the people in their group, you write down the questions for which they answer "yes", one per line. For example:

abcx
abcy
abcz
In this group, there are 6 questions to which anyone answered "yes": a, b, c, x, y, and z. (Duplicate answers to the same question don't count extra; each question counts at most once.)

Another group asks for your help, then another, and eventually you've collected answers from every group on the plane (your puzzle input). Each group's answers are separated by a blank line, and within each group, each person's answers are on a single line. For example:

abc

a
b
c

ab
ac

a
a
a
a

b
This list represents answers from five groups:

The first group contains one person who answered "yes" to 3 questions: a, b, and c.
The second group contains three people; combined, they answered "yes" to 3 questions: a, b, and c.
The third group contains two people; combined, they answered "yes" to 3 questions: a, b, and c.
The fourth group contains four people; combined, they answered "yes" to only 1 question, a.
The last group contains one person who answered "yes" to only 1 question, b.
In this example, the sum of these counts is 3 + 3 + 3 + 1 + 1 = 11.

For each group, count the number of questions to which anyone answered "yes". What is the sum of those counts?
*/
pub fn solve_1(input_dir: &Path) -> Result<()> {
    let answers = get_input(input_dir)?;
    let sum: u32 = answers.iter().map(|g| count_bits(g.iter().fold(0u32, |acc, i| acc | i))).sum();
    println!("\t{}", sum);
    Ok(())
}

/*
As you finish the last group's customs declaration, you notice that you misread one word in the instructions:

You don't need to identify the questions to which anyone answered "yes"; you need to identify the questions to which everyone answered "yes"!

Using the same example as above:

abc

a
b
c

ab
ac

a
a
a
a

b
This list represents answers from five groups:

In the first group, everyone (all 1 person) answered "yes" to 3 questions: a, b, and c.
In the second group, there is no question to which everyone answered "yes".
In the third group, everyone answered yes to only 1 question, a. Since some people did not answer "yes" to b or c, they don't count.
In the fourth group, everyone answered yes to only 1 question, a.
In the fifth group, everyone (all 1 person) answered "yes" to 1 question, b.
In this example, the sum of these counts is 3 + 0 + 1 + 1 + 1 = 6.

For each group, count the number of questions to which everyone answered "yes". What is the sum of those counts?
*/
pub fn solve_2(input_dir: &Path) -> Result<()> {
    let answers = get_input(input_dir)?;
    let sum: u32 = answers.iter().map(|g| count_bits(g.iter().fold(0xffffffffu32, |acc, i| acc & i))).sum();
    println!("\t{}", sum);
    Ok(())
}

//------------------------------------------ Tests ------------------------------------------

#[test]
fn test_count_bits() {
    assert_eq!(count_bits(0), 0, "No bits set.");
    assert_eq!(count_bits(1), 1, "One bit set.");
    assert_eq!(count_bits(1 << 24), 1, "One high bit set (24).");
    assert_eq!(count_bits(1 << 25), 1, "One high bit set (25).");
    assert_eq!(count_bits(1 << 26), 0, "One bit set out of range.");
    assert_eq!(count_bits(1 << 25 | 1), 2, "Two bits set.");
    assert_eq!(count_bits(1 << 25 | 1 << 24 | 1 << 14 | 1 << 13 | 1 << 1), 5, "Five bits set.");
    assert_eq!(count_bits(0xffffffff), 26, "Twenty six bits set.");
}
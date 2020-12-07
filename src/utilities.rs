use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use anyhow::{Result, Context};

pub fn get_input_lines<T: AsRef<Path>>(input_file: T) -> Result<Vec<String>> {
    let f = File::open(&input_file).with_context(|| format!("Failed to open input file '{}'.", input_file.as_ref().display()))?;
    let rdr = BufReader::new(f);
    let mut retval: Vec<String> = Vec::new();
    for line in rdr.lines() {
        retval.push(line.with_context(|| format!("Failed to read lines from input file '{}'.", input_file.as_ref().display()))?);
    }
    Ok(retval)
}
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use chrono::prelude::*;

pub fn read_file(day_num: u32) -> Result<Vec<String>, io::Error> {
    // read day specific text file line per line
    let current_dir = env::current_dir()?;
    let path = current_dir.join(format!("inputs\\day{day_num}.txt"));

    let file = match File::open(path) {
        Ok(file) => file,
        Err(err) => return Err(err),
    };
    let reader = io::BufReader::new(file);
    let lines: Result<Vec<String>, io::Error> = reader.lines().collect();
    
    return lines;
}
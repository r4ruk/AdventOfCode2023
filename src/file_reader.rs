use std::env;
use std::fs::{File, read};
use std::io::{self, BufRead, Read};

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

pub fn read_file_to_str(day_num: u32) -> String {
    // read day specific text file line per line
    let mut current_dir = env::current_dir();
    let path = current_dir.expect("Current dir not found for some reason").join(&*format!("inputs\\day{day_num}.txt"));
    let create_path = path.clone();
    let file = File::open(path).unwrap_or_else(|err| File::create(create_path).unwrap());
    let mut reader = io::BufReader::new(file);
    let mut return_string =String::new();
    reader.read_to_string(&mut return_string);

    return return_string;
}
use crate::file_reader::read_file_to_str;
use crate::solver;
extern crate aoc_lib;

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, _inputs: &Vec<String>) -> i128 {
        let input = read_file_to_str(15);
        let mut res = 0;
        for s in input.split(',') {
            res += calculate_value(s);
        }

        return res
    }

    fn solve_part2(&self, _inputs: &Vec<String>) -> i128 {
        let mut storage: Vec<Vec<(&str, u32)>> = vec![];
        let input = read_file_to_str(15);
        for _j in 0..=256{
            storage.push(Vec::new());
        }
        for s in input.split(',') {
            if s.contains("=") { // if it contains an =
                let lens_name: &str = s.split('=').collect::<Vec<_>>()[0];

                let map_index = calculate_value(lens_name) as usize;
                let working_box:&mut Vec<(&str, u32)> = &mut storage[map_index];
                let focal_length = s.split('=').collect::<Vec<_>>()[1].parse::<u32>().unwrap();

                let lens = (lens_name, focal_length);
                let mut found_index = -1;
                for (index, j) in working_box.iter().enumerate() {
                    if j.0 == lens_name {
                        found_index = index as i32;
                    }
                }
                if found_index > -1 {
                    working_box[found_index as usize] = lens;
                } else {
                    working_box.push(lens);
                }
            } else { // contains a -
                let lens_name = s.trim_matches('-');
                let map_index = calculate_value(lens_name) as usize;
                let working_box:&mut Vec<(&str, u32)> = &mut storage[map_index];

                let mut found_index = -1;
                for (index, j) in working_box.iter().enumerate() {
                    if j.0 == lens_name {
                        found_index = index as i32;
                    }
                }
                if found_index > -1 {
                    working_box.remove(found_index as usize);
                }
            }
        }
        let mut result = 0;
        // e.focal_length * (index in this range+1)  * (rangeindex over all ranges +1)   sum of all
        for i in 0..storage.len() {
            let ac_in = i as u32;
            result += storage[i].iter().enumerate().map(|(index, e)| e.1 * (index as u32 + 1) * (ac_in + 1)).sum::<u32>();
        }
        return result as i128
    }
}
fn calculate_value(input: &str) -> i128 {
    let mut result:i128 = 0;
    // run over all lines and calculate the value for each 0 found
    // thats the distance from end of the grid to the actual line in number
    for (_k, character) in input.chars().enumerate() {
        let i = character as u32;
        result += i as i128;
        result *= 17;
        result = result % 256
    }
    result
}
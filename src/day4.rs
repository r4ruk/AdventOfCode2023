use std::collections::HashMap;
use crate::{num_util, solver};

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, input: &Vec<String>) -> i128 {
        let mut res: i128 = 0;
        for line in input {
            // splitting magic
            let numberset: Vec<&str> = line.split(':').collect();
            let winning_mynum_split: Vec<&str> =  numberset[1].split('|').collect();
            let winning_numbers: Vec<&str> = winning_mynum_split[0].split_whitespace().collect();
            let my_nums: Vec<&str> = winning_mynum_split[1].split_whitespace().collect();

            // find matching numbers
            let matching_numbers = Self::find_matching_numbers(&winning_numbers, &my_nums);
            // if value numbers bigger than 0 add to the result
            if matching_numbers > 0 {
                res += 2i128.pow(matching_numbers - 1u32);
            }
        }
        return res as i128
    }

    fn solve_part2(&self, input: &Vec<String>) -> i128 {
        let mut values: HashMap<usize, i128> = HashMap::new();
        for (i, line) in input.iter().enumerate() {
            // add 1 for each line by default
            values.entry(i).or_insert(1);

            // split the stuff
            let numberset: Vec<&str> = line.split(':').collect();
            let winning_mynum_split: Vec<&str> =  numberset[1].split('|').collect();
            let winning_numbers: Vec<&str> = winning_mynum_split[0].split_whitespace().collect();
            let my_nums: Vec<&str> = winning_mynum_split[1].split_whitespace().collect();

            // search for amount of matching numbers
            let matching_nums = Self::find_matching_numbers(&winning_numbers, &my_nums);

            // for each following item add amount of items which i have of the actual item
            for j in 0..matching_nums as usize {
                let temp = *values.get(&i).unwrap();
                *values.entry(i+1+j).or_insert(1) += temp;
            }
        }

        // add all the values together
        return values.values().sum()
    }
}

impl SolverImpl {
    fn find_matching_numbers(winning_numbers: &Vec<&str>, my_nums: &Vec<&str>) -> u32{
        let mut temp_res: u32 = 0;

        for winning_number in winning_numbers {
            for num in my_nums {
                if num_util::parse_string_ref(num.trim()) == num_util::parse_string_ref(winning_number) {
                    temp_res += 1;
                }
            }
        }
        temp_res
    }
}
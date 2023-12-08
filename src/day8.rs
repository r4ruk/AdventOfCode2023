use std::collections::HashMap;
use crate::{solver};

extern crate aoc_lib;
use aoc_lib::math;

pub struct SolverImpl;

impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut sequences : HashMap<&str, (String, String)> = HashMap::new();
        let (instructions, others) = inputs.split_at(1);
        Self::prepare_sequences(&mut sequences, others);

        // initial startvalue
        let mut found_key = "AAA";
        let mut counter = 0;

        // continue until end is reached
        while found_key.to_string() != "ZZZ" {

            // walk the sequence given by direction chars
            for c in instructions[0].clone().chars() {
                let actual_element = sequences.get(found_key).unwrap();
                if c == 'L' {
                    found_key = &actual_element.0.trim()
                } else {
                    found_key = &actual_element.1.trim()
                }
                counter += 1;
            }
        }

        return counter as i128
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        let mut sequences : HashMap<&str, (String, String)> = HashMap::new();
        let (instructions, others) = inputs.split_at(1);
        Self::prepare_sequences(&mut sequences, others);

        let mut relevant_keys: Vec<String> = vec![];
        for (key, _) in sequences.iter() {
            if key.ends_with('A') {
                relevant_keys.push(key.to_string());
            }
        }

        let mut counter = 0;
        let mut found_key : &str;
        let mut numbers: Vec<i128> = vec![];

        // for all the relevant keys (starting with A
        for key in relevant_keys {
            found_key = &key;
            // while found key doesnt end on Z
            while found_key.ends_with('Z') == false {

                // walk the sequence given by direction chars
                for c in instructions[0].clone().chars() {
                    let actual_element = sequences.get(found_key).unwrap();
                    if c == 'L' {
                        found_key = &actual_element.0.trim()
                    } else {
                        found_key = &actual_element.1.trim()
                    }
                    counter += 1;
                }
            }
            numbers.push(counter);
            counter = 0;
        }

        // calculate the lowest common multiply of found numbers
        let res = math::basic::lcm(numbers);
        return res
    }
}

impl SolverImpl {
    fn prepare_sequences<'a>(sequences: &mut HashMap<&'a str, (String, String)>, others: &'a [String]) {
        for line in others {
            if line == &"".to_string() {
                continue
            }
            let goal_possib_split: Vec<&str> = line.split('=').collect();
            let possibilities: Vec<&str> = goal_possib_split[1].split(',').collect();
            let left_key = possibilities[0].to_string().replace('(', "").trim().to_string();
            let right_key = possibilities[1].to_string().replace(')', "").trim().to_string();
            sequences.insert(goal_possib_split[0].trim(), (left_key, right_key));
        }
    }
}
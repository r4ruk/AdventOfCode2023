use std::collections::HashMap;
use crate::{num_util, solver};

extern crate aoc_lib;
use aoc_lib::math;

pub struct SolverImpl;

impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut result: i128 = 0;
        for (index, line) in inputs.iter().enumerate() {
            let mut numbers: Vec<_> = line.split_whitespace().collect::<Vec<_>>().iter().map(|element| num_util::parse_string_ref(element)).collect();
            // first line of numbers
            result += extrapolate_last(&numbers)
        }

        return result
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        let mut result: i128 = 0;
        for (index, line) in inputs.iter().enumerate() {
            let mut numbers: Vec<_> = line.split_whitespace().collect::<Vec<_>>().iter().map(|element| num_util::parse_string_ref(element)).collect();

            result += extrapolate_first(&numbers)
        }

        return result
    }
}

fn extrapolate_last(x: &Vec<i128>) -> i128 {
    if x.iter().all(|n| *n == 0) {
        return 0;
    }
    let value_to_append = extrapolate_last(&generate_differences(x));

    x.last().unwrap() + value_to_append
}

fn extrapolate_first(x: &Vec<i128>) -> i128 {
    if x.iter().all(|n| *n == 0) {
        return 0;
    }
    let pre_value = extrapolate_first(&generate_differences(x));

    x.first().unwrap() - pre_value
}

fn generate_differences(x: &Vec<i128>) -> Vec<i128> {
    x.windows(2).map(|x| {
        x[1] - x[0]
    }).collect()
}
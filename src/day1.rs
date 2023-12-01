use crate::{num_util, solver};

pub struct SolverImpl;

impl solver::Solver for SolverImpl {

    fn solve_part1(&self, input: &Vec<String>) -> i128 {
        let mut res:i128 = 0;

        for line in input {
            let mut digits = vec![];

            // build vector of chars
            let chars = line.chars().collect::<Vec<_>>();

            // enumerate all chars and parse them to digits
            for (i, c) in chars.iter().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    // if its a digit push it into possible list
                    digits.push(d);
                    continue;
                }
            }
            // take first and last digit
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            res += num_util::parse_string(number);
        }
        return res
    }

    fn solve_part2(&self, input: &Vec<String>) -> i128 {
        let mut res:i128 = 0;
        let numbers = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        for line in input {
            let mut digits = vec![];

            // build vector of chars
            let chars = line.chars().collect::<Vec<_>>();

            // enumerate all chars and parse them to digits
            for (i, c) in chars.iter().enumerate() {
                if let Some(d) = c.to_digit(10) {
                    digits.push(d);
                    continue;
                }

                // construct a string from remaining characters
                let s = String::from_iter(&chars[i..chars.len()]);

                // check if remaining string starts with any of the numbers
                for (i, n) in numbers.iter().enumerate() {
                    if s.starts_with(n) {
                        // add index value + 1 (which is string value in digit)
                        digits.push(i as u32 + 1);
                    }
                }
            }
            let number = format!("{}{}", digits.first().unwrap(), digits.last().unwrap());
            res += num_util::parse_string(number);
        }
        return res
    }
}

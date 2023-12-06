use crate::{num_util, solver};

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {

        // parsing stuff to work with
        let (time, distance) = inputs.split_at(1);
        let times: Vec<_> = time[0].split(":").collect::<Vec<_>>()[1].split_whitespace().collect();
        let distances: Vec<_> = distance[0].split(":").collect::<Vec<_>>()[1].split_whitespace().collect();

        let mut possibilities:  Vec<i128> = vec![];
        // iterate over each possibility
        for (i, time) in times.iter().enumerate() {
            possibilities.push(calc_distance_covered(num_util::parse_string_ref(time), num_util::parse_string_ref(distances[i])))
        }
        // multiply all possibilities together for end result of part 1
        let mut res = 1;
        while possibilities.len() > 0 {
            res *= possibilities.pop().unwrap()
        }
        return res
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        // get the 2 lines seperate
        let (time, distance) = inputs.split_at(1);

        // splitting the line of times & merging numbers together to get 1 big number
        let total_time: String = time[0]
            .split(":")
            .nth(1)
            .unwrap_or("")
            .split_whitespace()
            .collect();

        // splitting the line of distances & merging numbers together to get 1 big number
        let total_distance: String = distance[0]
            .split(":")
            .nth(1)
            .unwrap_or("")
            .split_whitespace()
            .collect();

        return calc_distance_covered(num_util::parse_string(total_time), num_util::parse_string(total_distance))

    }
}

fn calc_distance_covered (mut time: i128, distance:i128) -> i128{
    let mut index = 0;
    let total_time = time;
    let mut possibility_counter = 0;

    while index < total_time {
        if is_winning_combination(index, time, distance) {
            possibility_counter += 1;
        }
        index +=1;
        time -= 1;
    }
    return possibility_counter;
}

// calculates if the combination of speed and remaining time is working on the given distance
fn is_winning_combination(speed: i128, time_remaining: i128, distance: i128) -> bool {
    let distance_covered = speed * time_remaining;
    if distance_covered > distance {
        return true
    }
    false
}

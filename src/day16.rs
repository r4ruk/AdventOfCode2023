use std::collections::{HashMap, HashSet};
use crate::solver;
extern crate aoc_lib;

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut res = 0;
        let mut grid: Vec<Vec<char>> = vec![];
        // for each line in the input
        Self::build_grid(inputs, &mut grid);

        // line, column, direction
        // whereas direction is
        // 1 = east
        // 2 = south
        // 3 = north
        // 4 = west
        let mut actual_directions: Vec<(i32, i32, i32)> = Vec::new();
        actual_directions.push((0, 0, 1));

        let mut set_of_tuples = HashSet::new();
        loop {
            Self::find_visited_tuples(&mut grid, &mut set_of_tuples, &mut actual_directions);
            if actual_directions.len() <= 0 {
                let res = distinct_count_of_sets(&set_of_tuples);
                return res as i128
            }
        }

        return 0
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        let mut grid: Vec<Vec<char>> = vec![];
        Self::build_grid(inputs, &mut grid);

        let directions = Self::build_starting_points(&mut grid);

        // run over each starting point and find the value of visited points in the grid.
        let mut actual_directions: Vec<_> = directions.into_iter().collect();
        let mut res_storage: Vec<i128> = vec![];
        for d in actual_directions {
            let mut set_of_tuples = HashSet::new();
            let mut single_direction_start: Vec<(i32, i32, i32)> = Vec::new();
            single_direction_start.push(d);
            loop {
                Self::find_visited_tuples(&mut grid, &mut set_of_tuples, &mut single_direction_start);
                if single_direction_start.len() <= 0 {
                    let res = distinct_count_of_sets(&set_of_tuples);
                    res_storage.push(res as i128);

                    break;
                }
            }
        }
        return res_storage.iter().cloned().max().unwrap()
    }
}

impl SolverImpl {
    fn build_grid(inputs: &Vec<String>, grid: &mut Vec<Vec<char>>) {
        // for each line in the input
        for (y, l) in inputs.iter().enumerate() {
            // add a new line to the input
            grid.push(Vec::new());
            for (_x, c) in l.chars().enumerate() {
                grid[y].push(c);
            }
        }
    }

    fn build_starting_points(grid: &mut Vec<Vec<char>>) -> HashSet<(i32, i32, i32)> {
        let mut directions: HashSet<(i32, i32, i32)> = HashSet::new();
        for (index, line) in grid.iter().enumerate() {
            let start_index = 0;
            let end_start_index = line.len() - 1;
            let left_tuple = (index as i32, start_index, 1);
            let right_tuple = (index as i32, end_start_index as i32, 4);
            directions.insert(left_tuple);
            directions.insert(right_tuple);
            if index == 1 || index == grid.len() - 1 {
                for (col_index, col) in line.iter().enumerate() {
                    let mut vertical_tuple;
                    if index == 0 {
                        vertical_tuple = (index as i32, col_index as i32, 2);
                    } else {
                        vertical_tuple = (index as i32, col_index as i32, 3);
                    }
                    directions.insert(vertical_tuple);
                }
            }
        }
        directions
    }

    fn find_visited_tuples(grid: &mut Vec<Vec<char>>, set_of_tuples: &mut HashSet<(i32, i32, i32)>, steps_store: &mut Vec<(i32, i32, i32)>) {
        for mut step in &mut steps_store.pop() {
            if step.0 < 0 || step.0 > (grid.len() - 1) as i32
                || (step.1 < 0 || step.1 > (grid[0].len() - 1) as i32)
                || set_of_tuples.insert((step.0, step.1, step.2)) == false {
                continue
            }
            match grid[step.0 as usize][step.1 as usize] {
                '/' => {
                    match step.2 {
                        1 => steps_store.push((step.0 - 1, step.1, 3)),
                        2 => steps_store.push((step.0, step.1 - 1, 4)),
                        3 => steps_store.push((step.0, step.1 + 1, 1)),
                        4 => steps_store.push((step.0 + 1, step.1, 2)),
                        _ => break
                    }
                },

                '\\' => {
                    match step.2 {
                        1 => steps_store.push((step.0 + 1, step.1, 2)),
                        2 => steps_store.push((step.0, step.1 + 1, 1)),
                        3 => steps_store.push((step.0, step.1 - 1, 4)),
                        4 => steps_store.push((step.0 - 1, step.1, 3)),
                        _ => break
                    }
                },
                '|' => {
                    match step.2 {
                        1 => {
                            steps_store.push((step.0 - 1, step.1, 3));
                            steps_store.push((step.0 + 1, step.1, 2));
                        },
                        2 => steps_store.push((step.0 + 1, step.1, 2)),
                        3 => steps_store.push((step.0 - 1, step.1, 3)),
                        4 => {
                            steps_store.push((step.0 - 1, step.1, 3));
                            steps_store.push((step.0 + 1, step.1, 2));
                        },
                        _ => break
                    }
                },
                '-' => {
                    match step.2 {
                        1 => { steps_store.push((step.0, step.1 + 1, 1)) },
                        2 => {
                            steps_store.push((step.0, step.1 + 1, 1));
                            steps_store.push((step.0, step.1 - 1, 4));
                        },
                        3 => {
                            steps_store.push((step.0, step.1 + 1, 1));
                            steps_store.push((step.0, step.1 - 1, 4));
                        },
                        4 => {
                            steps_store.push((step.0, step.1 - 1, 4));
                        },
                        _ => break
                    }
                },
                '.' => {
                    match step.2 {
                        1 => { steps_store.push((step.0, step.1 + 1, 1)) },
                        2 => { steps_store.push((step.0 + 1, step.1, 2)) },
                        3 => { steps_store.push((step.0 - 1, step.1, 3)) },
                        4 => { steps_store.push((step.0, step.1 - 1, 4)) },
                        _ => break
                    }
                },
                _ => break
            }
        }
    }
}

fn distinct_count_of_sets(v: &HashSet<(i32, i32,i32)>) -> i128 {
    let mut set_of_tuples = HashSet::new();

    for tuple in v.iter() {
        set_of_tuples.insert((tuple.0, tuple.1));
    }
    set_of_tuples.len() as i128
}
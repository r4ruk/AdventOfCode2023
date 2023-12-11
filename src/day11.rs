use std::collections::HashMap;
use crate::solver;
extern crate aoc_lib;
use aoc_lib::grid::manipulation::{self, Grid, Point};

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut grid = Grid::<String>::new(inputs, manipulation::DataSeparatingCriteria::EachChar);

         let result = walk_grid(&mut grid, 1);


        return result
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        let mut grid = Grid::<String>::new(inputs, manipulation::DataSeparatingCriteria::EachChar);

        let result = walk_grid(&mut grid, 999999);

       return result
    }
}

fn walk_grid(grid: &mut Grid<String>, amount_multiplier: i128) -> i128 {
    let mut  result:i128 = 0;
    let grid_length = grid.data.len();

    // find empty horizontal indexes
    let mut horizontal_indexes:Vec<usize> = Vec::new();
    for (index, line) in grid.data.iter().enumerate() {
        let mut is_empty = true;
        for p in line {
            if p.value.clone().unwrap() != ".".to_string()
            {
                is_empty = false;
                break // can be break as line is for sure not empty
            }
        }
        if is_empty {
             horizontal_indexes.push(index);

            // debugging purpose
            // println!("insert horizontal at {index}")
        }
    }

    // find empty vertical indexes
    let line_len = grid.data[0].len();
    let mut vertical_indexes = Vec::new();
    for x in 0..line_len {
        let mut is_empty = true;
        for y in 0..grid_length {
            let pnt: &Point<String> = &grid.data[y][x];
            if pnt.value.clone().unwrap() != "." {
                is_empty = false;
                break;
            }
        }
        if is_empty {
            vertical_indexes.push(x);

            // debugging purpose
            // println!("insert vertical at {x}")

        }
    }

    // store all "galaxies" coordinates (!= .)
    let mut point_storage:HashMap<i32, (i128, i128)> = HashMap::new();
    let mut index = 0;
    for (y_coord,line) in &mut grid.data.iter().enumerate() {
        let amount = find_amount_smaller(&horizontal_indexes, y_coord);
        let y_coord_corrected = y_coord as i128 + (amount as i128 * amount_multiplier);
        for (x_coord, col) in line.iter().enumerate() {
            if col.value.clone().unwrap() == "." {
                continue
            }
            else {
                let amount = find_amount_smaller(&vertical_indexes, x_coord);
                let x_coord_corrected = x_coord as i128 + (amount as i128 * amount_multiplier);
                point_storage.insert(index, (x_coord_corrected as i128, y_coord_corrected as i128));
                index += 1;
            }
        }
    }

    // calc the distance from each to each point.
    // lazy to implement visited logic => divide by 2 in the end
    let comparison_storage = point_storage.clone();
    for  element in point_storage {
        for l  in &comparison_storage {
            if element.0 == *l.0 {
                continue
            }
            else {
                result += manhattan_distance(element.1.0, element.1.1, l.1.0, l.1.1);
            }
        }
    }
    return result /2
}

// function used to find how many indexes were already before to calculate actual index
fn find_amount_smaller(indexes: &Vec<usize>, index: usize) -> i32{
    let mut amount = 0;
    for i in indexes {
        if index > *i {
            amount+=1;
        }
    }
    return amount
}


// classic manhattan distance calculation
fn manhattan_distance(x1: i128, y1: i128 , x2: i128, y2: i128) -> i128 {
    (x1 - x2).abs() + (y1 - y2).abs()
}
use crate::solver;
use std::collections::HashMap;

pub struct SolverImpl;

#[derive(Copy,Clone)]
struct Point {
    x: i32,
    y: i32
}
const ADJACENT_COORDS:[Point; 9]  = [Point{x: -1, y: -1},
Point{x: -1, y: -1},
Point{x: 0, y: -1},
Point{x: 1, y: -1},
Point{x: -1, y: 0},
Point{x: 1, y: 0},
Point{x: -1, y: 1},
Point{x: 0, y: 1},
Point{x: 1, y: 1}];

impl solver::Solver for SolverImpl {
    fn solve_part1(&self, input: &Vec<String>) -> i128 {
        let mut numbers:Vec<u32> = vec![];
        let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        let grid_height = grid.len();
        let grid_width = grid[0].len();

        // for every line
        for y in 0..grid_height {
            let mut has_adjacent_symbol = false;
            let mut current_number = 0;
            // for every character
            for x in 0..grid_width {
                let c: char = grid[y][x];
                let is_digit = c.is_digit(10);

                if is_digit {
                    let digit: u32 = c.to_digit(10).unwrap();
                    current_number *= 10;
                    current_number += digit;
                    // if 1 is adjacent to any digit of the number, it's supposed to be counted
                    has_adjacent_symbol = has_adjacent_symbol || is_symbol_adjacent(&grid, x as i32, y as i32);
                }

                // end of line or number
                if x == grid_width-1 || !is_digit {
                    if has_adjacent_symbol {
                        numbers.push(current_number)
                    }
                    // resetting values
                    current_number = 0;
                    has_adjacent_symbol = false
                }
            }
        }
        // summing all the numbers relevant
        let sum: u32 = numbers.iter().sum();
        return sum as i128
    }

    fn solve_part2(&self, input: &Vec<String>) -> i128 {
        let grid: Vec<Vec<char>> = input.iter().map(|line| line.chars().collect()).collect();
        let grid_height = grid.len();
        let grid_width = grid[0].len();

        let mut adjacent_gears:HashMap<(i32, i32), Point> = HashMap::new();
        let mut coordinates_with_values:HashMap<(i32, i32), Vec<i128>> = HashMap::new();

        // for every line
        for y in 0..grid_height {
            let mut current_number:i128 = 0;
            // for every character
            for x in 0..grid_width {
                let c: char = grid[y][x];
                let is_digit = c.is_digit(10);

                if is_digit {
                    let digit: i128 = c.to_digit(10).unwrap() as i128;
                    current_number *= 10;
                    current_number += digit;

                    // temporary storage for "gears" of actual number
                    let temp_adjacent = get_adjacent_gear(&grid, x as i32, y as i32);
                    if temp_adjacent.len() > 0 {
                        for pnt in temp_adjacent {
                            // only insert if its not yet in the storage
                            adjacent_gears.entry((pnt.x, pnt.y)).or_insert(pnt);
                        }
                    }
                }

                // actual number is finished
                if x == grid_width-1 || !is_digit {
                    if adjacent_gears.len() > 0 {
                        // if there is any gear value add it to the overall storage hashmap
                        for ((_,_), gear_pnt) in &adjacent_gears {
                            coordinates_with_values.entry((gear_pnt.x, gear_pnt.y)).or_insert(Vec::new()).push(current_number);
                        }
                    }
                    current_number = 0;
                    adjacent_gears.clear();
                }
            }
        }
        let mut result:i128 = 0;
        for  ((_, _), value) in &coordinates_with_values {
            // adding values where there is exact 2 values next to the gear.
            if value.len() == 2 {
                result += value[0] * value[1]
            }
        }
        return result;
    }
}


fn is_symbol_adjacent(grid: &Vec<Vec<char>>, x:i32, y:i32 ) -> bool{
    for point in ADJACENT_COORDS {
        let temp_pt = Point{x: x + point.x, y: y+point.y};

        if is_valid_coordinate(&temp_pt, grid){
            let c = grid.get(temp_pt.y as usize).unwrap().get(temp_pt.x as usize).unwrap();

            // check if another symbol than '.' or number is adjacent
            if is_symbol(*c) {
                return true
            }
        }
    }
    return false;
}

fn get_adjacent_gear(grid: &Vec<Vec<char>>, x:i32, y:i32 ) -> Vec<Point> {
    let mut adjacent_gears:Vec<Point>=vec![];

    for point in ADJACENT_COORDS {
        let temp_pt = Point { x: x + point.x, y: y + point.y };
        // ensure valid coordinate
        if is_valid_coordinate(&temp_pt, grid) {
            // get actual grid character value
            let c = grid.get(temp_pt.y as usize).unwrap().get(temp_pt.x as usize).unwrap();

            if is_gear(*c) {
                adjacent_gears.push(temp_pt)
            }
        }
    }
    return adjacent_gears;
}
fn is_symbol(c: char) -> bool{
    return !c.is_digit(10) && c != '.';
}

fn is_gear(c: char) -> bool{
    return c == '*';
}

fn is_valid_coordinate(pt: &Point, grid: &Vec<Vec<char>> ) -> bool {
    return pt.x >= 0
        && pt.y >= 0
        && pt.y < grid.len() as i32
        && pt.x < grid[0].len() as i32
}

use std::collections::HashMap;
use crate::solver;
extern crate aoc_lib;
enum Direction {
    North,
    West,
    South,
    East
}

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128 {
        let mut tilting_grid: Vec<Vec<char>> = vec![];
        // for each line in the input
        for (y, l) in inputs.iter().enumerate() {
            tilting_grid.push(Vec::new());
            // add a new line to the input
            for (_x, c) in l.chars().enumerate() {
                tilting_grid[y].push(c);
            }
        }
        tilt_grid(&mut tilting_grid, Direction::North);
        let result = calculate_value(&tilting_grid);
        return result
    }

    fn solve_part2(&self, inputs: &Vec<String>) -> i128 {
        let mut tilting_grid: Vec<Vec<char>> = vec![];
        // for each line in the input
        for (y, l) in inputs.iter().enumerate() {
            tilting_grid.push(Vec::new());
            // add a new line to the input
            for (_x, c) in l.chars().enumerate() {
                tilting_grid[y].push(c);
            }
        }

        let mut storage_map: HashMap<i128, i32> = HashMap::new();
        for _i in 0..110 {
            tilt_grid(&mut tilting_grid, Direction::North);
            tilt_grid(&mut tilting_grid, Direction::West);
            tilt_grid(&mut tilting_grid, Direction::South);
            tilt_grid(&mut tilting_grid, Direction::East);

            let val = calculate_value(&tilting_grid);

            *storage_map.entry(val).or_insert(1) += 1;
        }

        let result = storage_map.iter().max_by_key(|e| e.1).unwrap();

        return *result.0
    }
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn calculate_value(grid: &Vec<Vec<char>>) -> i128 {
    let mut result:i128 = 0;
    // run over all lines and calculate the value for each 0 found
    // thats the distance from end of the grid to the actual line in number
    for (index, line) in grid.iter().enumerate() {
        for character in line {
            if *character == 'O' {
                result += line.len() as i128 - index as i128;
            }
        }
    }
    result
}

fn tilt_grid(grid: &mut Vec<Vec<char>>, dir: Direction) {
    match dir {
        Direction::North => {
            for (y, line) in grid.clone().iter().enumerate() {
                for (x, c) in line.iter().enumerate() {
                    if *c == 'O' {
                        if y == 0 {continue}
                        let mut temp_y = y - 1;
                        while temp_y > 0
                            && grid[temp_y][x] != 'O'
                            && grid[temp_y][x] != '#' {
                            temp_y -= 1;
                        }
                        if temp_y != 0 {
                            temp_y += 1;
                        } else if temp_y == 0 && grid[temp_y][x] != '.' {
                            temp_y += 1;
                        }
                        if temp_y != y {
                            let element = grid[temp_y].get_mut(x);
                            match element {
                                Some(element) => {
                                    *element = *c;
                                    // push default value to actual position
                                    *grid[y].get_mut(x).unwrap() = '.';
                                },
                                None => println!("found nothing on element")
                            }
                        }
                    }
                }
            }
        },
        Direction::West => {
            for (y, line) in grid.clone().iter().enumerate() {
                for (x, c) in line.iter().enumerate() {
                    if *c == 'O' {
                        if x == 0 {continue}
                        let mut temp_x = x - 1;
                        while temp_x > 0
                            && grid[y][temp_x] != 'O'
                            && grid[y][temp_x] != '#' {
                            temp_x -= 1;
                        }
                        if temp_x != 0 {
                            temp_x += 1;
                        } else if temp_x == 0 && grid[y][temp_x] != '.' {
                            temp_x += 1;
                        }

                        if temp_x != x {
                            let element = grid[y].get_mut(temp_x);
                            match element {
                                Some(element) => {
                                    *element = *c;
                                    // push default value to actual position

                                    *grid[y].get_mut(x).unwrap() = '.';
                                },
                                None => println!("found nothing on element")
                            }
                        }
                    }
                }
            }
        },
        Direction::South => {
            for (y, line) in grid.clone().iter().rev().enumerate() {
                for (x, c) in line.iter().enumerate() {
                    if *c == 'O' {
                        if y == 0 {continue}
                        let actual_index = grid.len() - 1 - y;
                        let mut temp_y = actual_index + 1;
                        while temp_y < grid.len()
                            && grid[temp_y][x] != 'O'
                            && grid[temp_y][x] != '#' {
                            temp_y += 1;
                        }
                        if temp_y != grid.len()-1 {
                            temp_y -= 1;
                        } else if temp_y == grid.len()-1 && grid[temp_y][x] != '.' {
                            temp_y -= 1;
                        }

                        if temp_y != actual_index {
                            let element = grid[temp_y].get_mut(x);
                            match element {
                                Some(element) => {
                                    *element = *c;
                                    // push default value to actual position
                                    *grid[actual_index].get_mut(x).unwrap() = '.';
                                },
                                None => println!("found nothing on element")
                            }
                        }
                    }
                }
            }
        },
        Direction::East => {
            for (y, line) in grid.clone().iter().enumerate() {
                for (x, c) in line.iter().rev().enumerate() {
                    if *c == 'O' {
                        if x == 0 {continue}
                        let actual_index = line.len() -1 - x;
                        let mut temp_x = actual_index+1;
                        while temp_x < line.len()
                            && grid[y][temp_x] != 'O'
                            && grid[y][temp_x] != '#' {
                            temp_x += 1;
                        }
                        if temp_x != line.len()-1 {
                            temp_x -= 1;
                        } else if temp_x == line.len()-1 && grid[y][temp_x] != '.' {
                            temp_x -= 1;
                        }

                        if temp_x != actual_index {
                            let element = grid[y].get_mut(temp_x);
                            match element {
                                Some(element) => {
                                    *element = *c;
                                    // push default value to actual position
                                    * grid[y].get_mut(actual_index).unwrap() = '.';
                                },
                                None => println!("found nothing on element")
                            }
                        }
                    }
                }
            }
        }
    }
}
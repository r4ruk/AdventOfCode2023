use crate::file_reader::read_file_to_str;
use crate::solver;
extern crate aoc_lib;
use aoc_lib::math::geo::Coord;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Instruction {
    direction: Direction,
    steps: usize,
}

pub struct SolverImpl;
impl solver::Solver for SolverImpl {
    fn solve_part1(&self, _inputs: &Vec<String>) -> i128 {
        let input = read_file_to_str(18);

        let instructions: Vec<Instruction> = input.lines()
            .map(|line| {
                let line = line.split(' ').collect::<Vec<&str>>();
                Instruction {
                    direction: match line[0] {
                        "U" => Direction::Up,
                        "D" => Direction::Down,
                        "L" => Direction::Left,
                        "R" => Direction::Right,
                        _ => panic!("Invalid direction"),
                    },
                    steps: line[1]
                        .parse::<usize>()
                        .expect("Second value should be a number of steps"),
                }
            })
            .collect();

        let curr_pos = Coord { x: 0, y: 0 };
        let mut coords: Vec<Coord> = vec![curr_pos];

        Self::collect_coords(instructions, curr_pos, &mut coords);

        let area = aoc_lib::math::geo::shoelace_formula(&coords);
        let perimeter = (coords.len() - 1) as i128;

        let res = area + perimeter;
        return res as i128
    }

    fn solve_part2(&self, _inputs: &Vec<String>) -> i128 {
        let input = read_file_to_str(18);

        let instructions: Vec<Instruction> = input
            .lines()
            .map(|line| {
                let line = line.split(' ').collect::<Vec<&str>>();
                coulor_instruction_translation(line[2][1..8].to_string())
            })
            .collect();

        let curr_pos = Coord { x: 0, y: 0 };
        let mut coords: Vec<Coord> = vec![curr_pos];

        Self::collect_coords(instructions, curr_pos, &mut coords);

        let area = aoc_lib::math::geo::shoelace_formula(&coords);
        let perimeter = (coords.len() - 1) as i128;

        return area + perimeter
    }
}

impl SolverImpl {
    fn collect_coords(instructions: Vec<Instruction>, mut curr_pos: Coord, coords: &mut Vec<Coord>) {
        for instruction in instructions {
            for _ in 0..instruction.steps {
                match instruction.direction {
                    Direction::Up => {
                        curr_pos.y -= 1;
                    }
                    Direction::Down => {
                        curr_pos.y += 1;
                    }
                    Direction::Left => {
                        curr_pos.x -= 1;
                    }
                    Direction::Right => {
                        curr_pos.x += 1;
                    }
                }
                coords.push(curr_pos);
            }
        }
    }
}

fn coulor_instruction_translation(colour: String) -> Instruction {
    Instruction {
        direction: match colour.chars().nth(6).expect("Invalid") {
            '0' => Direction::Right,
            '1' => Direction::Down,
            '2' => Direction::Left,
            '3' => Direction::Up,
            _ => panic!("Invalid"),
        },
        steps: usize::from_str_radix(&colour[1..6], 16).expect("Invalid")
    }
}
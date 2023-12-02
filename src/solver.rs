extern crate stopwatch;
use stopwatch::{Stopwatch};

pub trait Solver {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128;
    fn solve_part2(&self, inputs: &Vec<String>) -> i128;
}

pub fn run_solver(solver: &dyn Solver, inputs: Vec<String>) {
    let mut sw1 = Stopwatch::start_new();
    let result_part1 = solver.solve_part1(&inputs);
    sw1.stop();

    let mut sw2 = Stopwatch::start_new();
    let result_part2 = solver.solve_part2(&inputs);
    sw2.stop();

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);

    // timing information
    println!("");
    let mut str_seconds = String::from("microseconds");
    let p1_micro =  sw1.elapsed().as_micros() as f64;
    let mut p1 = p1_micro;

    if p1_micro / 1000.0 > 1.0 {
        p1 = p1_micro/1000.0;
        str_seconds = String::from("milliseconds");
    }
    println!("time for part1: {p1} {str_seconds}");

    let p2_micro =  sw2.elapsed().as_micros()  as f64;
    let mut p2 = p2_micro;
    if p2_micro / 1000.0 > 1.0 {
        p2 = p2_micro/1000.0;
        str_seconds = String::from("milliseconds");
    }
    println!("time for part2: {p2} {str_seconds}");
}

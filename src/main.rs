
mod file_reader;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod num_util;
mod day_helper;
mod solver;

fn main() {
    let day: u32 = day_helper::get_day();
    let result = file_reader::read_file(day);

    let inputs: Vec<String> = match result {
        Ok(res) => res,
        Err(err) => {
            println!("error: {err}");
            vec![]
        }
    };

    let solver: Box<dyn solver::Solver> = match day {
        1 => Box::new(day1::SolverImpl),
        2 => Box::new(day2::SolverImpl),
        3 => Box::new(day3::SolverImpl),
        4 => Box::new(day4::SolverImpl),
        5 => Box::new(day5::SolverImpl),
        6 => Box::new(day6::SolverImpl),
        7 => Box::new(day7::SolverImpl),
        _ => {
            println!("Solver for day {day} has not yet been implemented.");
            return;
        }
    };
    solver::run_solver(&*solver, inputs);
}

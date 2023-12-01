
mod file_reader;
mod day1;
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
        _ => {
            println!("Solver for day {day} has not yet been implemented.");
            return;
        }
    };
    solver::run_solver(&*solver, inputs);
}
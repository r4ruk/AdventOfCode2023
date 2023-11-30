pub trait Solver {
    fn solve_part1(&self, inputs: &Vec<String>) -> i128;
    fn solve_part2(&self, inputs: &Vec<String>) -> i128;
}

pub fn run_solver(solver: &dyn Solver, inputs: Vec<String>) {
    let result_part1 = solver.solve_part1(&inputs);
    let result_part2 = solver.solve_part2(&inputs);

    println!("Part 1: {}", result_part1);
    println!("Part 2: {}", result_part2);
}

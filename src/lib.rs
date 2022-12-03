use itertools::Itertools;
use std::thread;
use std::time::Instant;

// pub type Solver = dyn Fn(&[String]) -> usize;

pub struct ProblemSolution<'a> {
    solver: &'a (dyn Fn(&[&str]) -> usize + Sync),
    sample_solution: usize,
}

impl<'a> ProblemSolution<'a> {
    pub fn new(solver: &'a (dyn Fn(&[&str]) -> usize + Sync), sample_solution: usize) -> Self {
        Self {
            solver,
            sample_solution,
        }
    }

    pub fn run(&self, input: &[&str]) -> usize {
        (self.solver)(input)
    }
}

fn get_lines(file: &str) -> Vec<&str> {
    file.lines().collect_vec()
}

pub fn run_all(part_one: ProblemSolution, part_two: ProblemSolution, sample: &str, input: &str, sample_2: &str, input_2: &str) {
    let sample = get_lines(sample);
    let real = get_lines(input);
    let result = part_one.run(&sample);
    assert_eq!(result, part_one.sample_solution);
    let start = Instant::now();
    let result = part_one.run(&real);
    println!("Part one: {:?}, took {:?}", result, start.elapsed());

    let sample_2 = get_lines(sample_2);
    let real_2 = get_lines(input_2);
    let result = part_two.run(&sample_2);
    assert_eq!(result, part_two.sample_solution);
    let start = Instant::now();
    let result = part_two.run(&real_2);
    println!("Part one: {:?}, took {:?}", result, start.elapsed());

}

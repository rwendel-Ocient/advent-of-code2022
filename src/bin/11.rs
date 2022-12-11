use std::{collections::VecDeque, iter::FromIterator};

use std::fmt;
use itertools::Itertools;
use prime_factorization::Factorization;


fn main() {
    let sample = include_str!("../../samples/11.txt");
    let input = include_str!("../../inputs/11.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 10605 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 2713310158 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

struct Monkey<FnOperation, FnTest> 
where 
    FnOperation: Fn(u128) -> u128,
    FnTest: Fn(u128) -> (usize, u128)
{
    pub queue: VecDeque<u128>,
    pub operation: FnOperation,
    pub test: FnTest,
    pub num_inspections: usize,
}
impl<FnOperation, FnTest> fmt::Debug for Monkey<FnOperation, FnTest>
where 
    FnOperation: Fn(u128) -> u128,
    FnTest: Fn(u128) -> (usize, u128)
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.num_inspections)
    }
}

impl<FnOperation, FnTest> Monkey<FnOperation, FnTest>
where
    FnOperation: Fn(u128) -> u128,
    FnTest: Fn(u128) -> (usize, u128)
{
    fn new(q: Vec<u128>, op: FnOperation, test: FnTest ) -> Self {
        Self { 
            queue: VecDeque::from(q), 
            operation: op, 
            test: test,
            num_inspections: 0
        }
    }
}

fn part_one(lines: &[&str]) -> usize {
    let num_monkies = (lines.len()+1)/ 7;
    let mut monkey_vec = Vec::new();

    let mut lines_iter = lines.iter();

    for _ in 0..num_monkies {
        assert!(lines_iter.next().unwrap().starts_with("Monkey "));

        let starting_items = lines_iter.next().unwrap().trim();
        assert!(starting_items.starts_with("Starting items:"));
        let vec_starting = starting_items.split(":").skip(1).next().unwrap().replace(" ", "").split(",").map(|x| { x.parse::<u128>().unwrap()}).collect_vec();

        let operation = lines_iter.next().unwrap().trim();
        assert!(operation.starts_with("Operation:"));
        let operation_interesting_idx = operation.find("new = old").unwrap() + 9;
        let operation_remainder = operation[operation_interesting_idx..].trim().split(" ").collect_vec();
        let operation_second_arg = operation_remainder[1].parse::<u128>();
        let fn_op: Box<dyn Fn(u128) -> u128> = match operation_remainder[0] {
            "+" => {
                match operation_second_arg {
                    Err(_) => {
                        Box::new(|a: u128| a + a)
                    }
                    Ok(x) => {
                        Box::new(move |a: u128| a + x)
                    }
                }
            }
            "*" => {
                match operation_second_arg {
                    Err(_) => {
                        Box::new(|a: u128| a * a)
                    }
                    Ok(x) => {
                        Box::new(move |a: u128| a * x)
                    }
                }

            }
            _ => {
                panic!("my parsing, its very sick")
            }
        };

        let test_divis = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<u128>().unwrap();
        let test_true = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<usize>().unwrap();
        let test_false = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<usize>().unwrap();

        let test_op = move |a: u128| {
            if a % test_divis == 0 {
                (test_true, a )
            } else {
                (test_false, a)
            }
        };

        lines_iter.next();


        monkey_vec.push(Monkey::new(vec_starting, fn_op, test_op));
    }

    println!("{:?}", monkey_vec);

    // Do 20 rounds
    for i in 0..20 {
        for m in 0..num_monkies {
            for _ in 0..monkey_vec[m].queue.len(){
                let working_item = monkey_vec[m].queue.pop_front().unwrap();
                let heightened_worry = (monkey_vec[m].operation)(working_item);
                monkey_vec[m].num_inspections +=1;
                let divided_worry = heightened_worry/3;
                let (monkey_index, dived) = (monkey_vec[m].test)(divided_worry);
                assert!(monkey_index < num_monkies);
                monkey_vec[monkey_index].queue.push_back(dived);
            }
        }

        //println!("after round {} {:?}",i+1, monkey_vec);
    }

    let mut sorted_vec = monkey_vec.iter().map(|m| m.num_inspections).collect_vec();
    sorted_vec.sort();
    println!("sorted: {:?}", sorted_vec);
    let mut rev_iter = sorted_vec.into_iter().rev();
    rev_iter.next().unwrap() * rev_iter.next().unwrap()
}

fn part_two(lines: &[&str]) -> usize {
    let num_monkies = (lines.len()+1)/ 7;
    let mut monkey_vec = Vec::new();
    let mut all_divis = Vec::new();

    let mut lines_iter = lines.iter();

    for _ in 0..num_monkies {
        assert!(lines_iter.next().unwrap().starts_with("Monkey "));

        let starting_items = lines_iter.next().unwrap().trim();
        assert!(starting_items.starts_with("Starting items:"));
        let vec_starting = starting_items.split(":").skip(1).next().unwrap().replace(" ", "").split(",").map(|x| { x.parse::<u128>().unwrap()}).collect_vec();

        let operation = lines_iter.next().unwrap().trim();
        assert!(operation.starts_with("Operation:"));
        let operation_interesting_idx = operation.find("new = old").unwrap() + 9;
        let operation_remainder = operation[operation_interesting_idx..].trim().split(" ").collect_vec();
        let operation_second_arg = operation_remainder[1].parse::<u128>();
        let fn_op: Box<dyn Fn(u128) -> u128> = match operation_remainder[0] {
            "+" => {
                match operation_second_arg {
                    Err(_) => {
                        Box::new(|a: u128| a + a)
                    }
                    Ok(x) => {
                        Box::new(move |a: u128| a + x)
                    }
                }
            }
            "*" => {
                match operation_second_arg {
                    Err(_) => {
                        Box::new(|a: u128| a * a)
                    }
                    Ok(x) => {
                        Box::new(move |a: u128| {
                            a * x
                    })
                    }
                }

            }
            _ => {
                panic!("my parsing, its very sick")
            }
        };

        let test_divis = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<u128>().unwrap();
        let test_true = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<usize>().unwrap();
        let test_false = lines_iter.next().unwrap().split(" ").last().unwrap().to_string().parse::<usize>().unwrap();

        let test_op = move |a: u128| {
            if a % test_divis == 0 {
                (test_true, a)
            } else {
                (test_false, a)
            }
        };

        all_divis.push(test_divis);


        lines_iter.next();


        monkey_vec.push(Monkey::new(vec_starting, fn_op, test_op));
    }

    //println!("{:?}", monkey_vec);
    let superdivisor: u128 = all_divis.iter().product();

    // Do 20 rounds
    for i in 0..10000 {
        for m in 0..num_monkies {
            for _ in 0..monkey_vec[m].queue.len(){
                monkey_vec[m].num_inspections +=1;

                let working_item = monkey_vec[m].queue.pop_front().unwrap();
                let heightened_worry = (monkey_vec[m].operation)(working_item) % superdivisor;

                let (monkey_index, dived) = (monkey_vec[m].test)(heightened_worry as u128);

                monkey_vec[monkey_index].queue.push_back(heightened_worry as u128);
            }
        }

        println!("after round {} {:?}",i+1, monkey_vec);
    }

    let mut sorted_vec = monkey_vec.iter().map(|m| m.num_inspections).collect_vec();
    sorted_vec.sort();
    println!("sorted: {:?}", sorted_vec);
    let mut rev_iter = sorted_vec.into_iter().rev();
    rev_iter.next().unwrap() * rev_iter.next().unwrap()
}

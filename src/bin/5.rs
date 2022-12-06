#![feature(iter_next_chunk)]
#[macro_use] extern crate scan_fmt;
use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/5.txt");
    let input = include_str!("../../inputs/5.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 0 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 0 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

fn part_one(lines: &[&str]) -> usize {
    let len = (lines[0].len() + 1)/4;
    let starting =  lines.iter().map_while(| line| {
        assert!(line.len() > 2);
        let c = line.chars().nth(1).unwrap();
        if  c == '1' {
            None
        }
        else {
            Some(line)
        }
    }).collect::<Vec<_>>();
    let max_height = starting.len();
    println!("got len{} with max_height{:?}", len, max_height);
    let mut cargo:Vec<Vec<String>> = vec![vec![]; len + 1];
    starting.iter().rev().for_each(|line|{
        //println!("{}", line);
        let mut line_chars = line.chars();
        for i in 1..len+1 {
            let line_as_char = line_chars.next_chunk::<4>();
            //println!("{:?}", line_as_char);
            match line_as_char {
                Ok(x) => {
                    if x[1] != ' ' {
                        cargo[i].push(x[1].to_string())
                }}
                Err(x) => {if x.as_slice()[1] != ' ' { cargo[i].push(x.as_slice()[1].to_string())}}
            }

        }
    });
    println!("cargo {:?}", cargo);

    lines.iter().skip(max_height+2).for_each(|line|{
        println!("{}", line);
        if let Ok((num, from, to)) = scan_fmt!(line, "move {d} from {d} to {d}", u32, usize, usize) {
            for _i in 0..num {
                let s = cargo[from].pop();
                match s {
                    Some(ss) => {cargo[to].push(ss)}
                    None => {println!("{:?}", cargo)}
                }
            }
        }
    });

    let mut output_string = String::from("");
    for i in 1..len+1 {
        output_string += cargo[i].last().unwrap();
    }
    println!("{}", output_string);


    //lines.iter().next_chunk::<1>()
    0
}

fn part_two(lines: &[&str]) -> usize {
    let len = (lines[0].len() + 1)/4;
    let starting =  lines.iter().map_while(| line| {
        assert!(line.len() > 2);
        let c = line.chars().nth(1).unwrap();
        if  c == '1' {
            None
        }
        else {
            Some(line)
        }
    }).collect::<Vec<_>>();
    let max_height = starting.len();
    println!("got len{} with max_height{:?}", len, max_height);
    let mut cargo:Vec<Vec<String>> = vec![vec![]; len + 1];
    starting.iter().rev().for_each(|line|{
        //println!("{}", line);
        let mut line_chars = line.chars();
        for i in 1..len+1 {
            let line_as_char = line_chars.next_chunk::<4>();
            //println!("{:?}", line_as_char);
            match line_as_char {
                Ok(x) => {
                    if x[1] != ' ' {
                        cargo[i].push(x[1].to_string())
                }}
                Err(x) => {if x.as_slice()[1] != ' ' { cargo[i].push(x.as_slice()[1].to_string())}}
            }

        }
    });
    println!("cargo {:?}", cargo);

    lines.iter().skip(max_height+2).for_each(|line|{
        //println!("{}", line);
        if let Ok((num, from, to)) = scan_fmt!(line, "move {d} from {d} to {d}", u32, usize, usize) {
            let end_idx = cargo[from].len();
            let start_idx = end_idx - num as usize;
            for i in start_idx..end_idx {
                let pls_work = cargo[from][i].clone();
                cargo[to].push(pls_work);
            }
            for i in 0..num {
                cargo[from].pop();
            }
        }
    });

    let mut output_string = String::from("");
    for i in 1..len+1 {
        output_string += cargo[i].last().unwrap();
    }
    println!("{}", output_string);


    //lines.iter().next_chunk::<1>()
    0
}

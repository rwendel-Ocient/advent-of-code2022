#![feature(array_chunks)]
use std::convert::TryInto;

use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/3.txt");
    let sample_2 = include_str!("../../samples/3_2.txt");
    let input = include_str!("../../inputs/3.txt");
    let input_2 = include_str!("../../inputs/3_2.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 157 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 70 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample_2, input_2);
}

fn to_priority(letter: &char) -> usize {
    let p = *letter as usize;
    if p >= 97 {
        p - 96
    } else {
        p - 38
    }
}

fn part_one(lines: &[&str]) -> usize {
    assert!(to_priority(&"A".chars().nth(0).unwrap()) == 27);
    assert!(to_priority(&"L".chars().nth(0).unwrap()) == 38);
    assert!(to_priority(&"a".chars().nth(0).unwrap()) == 1);
    lines
        .iter()
        .map(|line| {
            let length = line.len();
            assert!(length % 2 == 0);
            let chars_priority = line.chars().map(|c| to_priority(&c));
            let mut check_against: [i32; 53] = [0; 53];
            let mut found = None;
            chars_priority.enumerate().for_each(|(i, x)| {
                if i < length / 2 {
                    // First half
                    check_against[x] = 1;
                } else {
                    // second half
                    if check_against[x] == 1 {
                        if found.is_none() {
                            found = Some(x)
                        } else {
                            assert!(found.unwrap() == x)
                        }
                    }
                }
            });
            assert!(found.is_some());

            found.unwrap()
        })
        .sum()
}

fn part_two(lines: &[&str]) -> usize {
    assert!(lines.len() % 3 == 0);
    lines.array_chunks::<3>()
        .map(|line_chunk| {
            let mut check_against: [u32; 53] = [0; 53];
            let mut found = None;
            line_chunk.into_iter().enumerate().for_each(|(i, line)|{
                println!("on line {}", line);
                let chars_priority = line.chars().map(|c| to_priority(&c));
                chars_priority.for_each(|x| {
                        assert!(i < 3);
                        if x == 18 {
                            println!("on r, check_against: {}", check_against[x]);
                        }
                        check_against[x] = check_against[x] | (1 << (i ));
                        assert!(7 == 0b111u32);
                        if check_against[x] == 0b111u32 {
                            if found.is_none() {
                                found = Some(x)
                            } else {
                                assert!(found.unwrap() == x)
                            }
                        }
                        if x == 18 {
                            println!("on r, after check_against: {}", check_against[x]);
                        }
                    });
                });
                assert!(found.is_some());

                found.unwrap()
            }).sum()
}

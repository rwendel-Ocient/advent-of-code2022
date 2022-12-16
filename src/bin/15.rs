use std::convert::TryInto;

use itertools::Itertools;
#[macro_use]
extern crate scan_fmt;

fn main() {
    let sample = include_str!("../../samples/15.txt");
    let input = include_str!("../../inputs/15.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 26 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 56000011 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

#[derive(Debug)]
struct PointBeacon {
    point: (i64, i64),
    beacon: (i64, i64),
    dist: i64,
}

impl PointBeacon {
    fn new(point: (i64, i64), beacon: (i64, i64)) -> Self {
        PointBeacon {
            point: point,
            beacon: beacon,
            dist: (point.0 - beacon.0).abs() + (point.1 - beacon.1).abs(),
        }
    }
    fn hit(&self, x: i64, y: i64) -> bool {
        ((self.point.0 - x).abs() + (self.point.1 - y).abs() <= self.dist) && (x, y) != self.beacon
    }

    fn hit_and_jump(&self, x: i64, y: i64) -> (bool, usize) {
        let mut jump: usize = 1;

        let hit = (self.point.0 - x).abs() + (self.point.1 - y).abs() <= self.dist;
        if hit && x < self.point.0 {
            jump = (self.point.0 as usize - x as usize) * 2;
        }
        (hit, jump)
    }

    fn coord_jump(&self, x: i64, y: i64) -> usize {
        let mut coord_jump: usize = 0;

        let hit = (self.point.0 - x).abs() + (self.point.1 - y).abs() <= self.dist;
        if hit {
            coord_jump =
                self.point.0 as usize + self.dist as usize - (self.point.1 - y).abs() as usize;
        }
        coord_jump
    }
}

fn part_one(lines: &[&str]) -> usize {
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let pb = lines
        .iter()
        .map(|line| {
            if let Ok((x, y, bx, by)) = scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i64,
                i64,
                i64,
                i64
            ) {
                max_x = std::cmp::max(x, std::cmp::max(bx, max_x));
                max_y = std::cmp::max(y, std::cmp::max(by, max_y));
                min_x = std::cmp::min(x, std::cmp::min(bx, min_x));
                min_y = std::cmp::min(y, std::cmp::min(by, min_y));
                PointBeacon::new((x, y), (bx, by))
            } else {
                panic!("bad parse");
            }
        })
        .collect_vec();

    println!("{:?}", pb);
    println!(
        "bounds: max_x{} max_y{} min_x{} min_y{}",
        max_x, max_y, min_x, min_y
    );

    let mut target_row = 10;
    if pb.len() != 14 {
        target_row = 2000000; // real
    }

    let mut num_hit = 0;
    // God tier big brain for loop below.
    for x in (min_x - 10000000)..(max_x + 10000000) {
        if pb.iter().fold(false, |acc, p| acc || p.hit(x, target_row)) {
            //println!("hit x:{} y:{}", x, target_row);
            num_hit += 1;
        }
        if x % 100000 == 0 {
            println!("on x: {}, num_hit {} with max: {}", x, num_hit, max_x);
        }
    }

    num_hit
}

fn part_two(lines: &[&str]) -> usize {
    let mut max_x = std::i64::MIN;
    let mut max_y = std::i64::MIN;
    let mut min_x = std::i64::MAX;
    let mut min_y = std::i64::MAX;
    let pb = lines
        .iter()
        .map(|line| {
            if let Ok((x, y, bx, by)) = scan_fmt!(
                line,
                "Sensor at x={d}, y={d}: closest beacon is at x={d}, y={d}",
                i64,
                i64,
                i64,
                i64
            ) {
                max_x = std::cmp::max(x, std::cmp::max(bx, max_x));
                max_y = std::cmp::max(y, std::cmp::max(by, max_y));
                min_x = std::cmp::min(x, std::cmp::min(bx, min_x));
                min_y = std::cmp::min(y, std::cmp::min(by, min_y));
                PointBeacon::new((x, y), (bx, by))
            } else {
                panic!("bad parse");
            }
        })
        .collect_vec();

    println!("{:?}", pb);
    println!(
        "bounds: max_x{} max_y{} min_x{} min_y{}",
        max_x, max_y, min_x, min_y
    );

    let mut bounds = 20;
    if pb.len() != 14 {
        bounds = 4000000; // real
    }

    let mut not_hit_x = 0;
    let mut not_hit_y = 0;
    //for y in 0..bounds {
    //    let mut x: usize = 0;
    //    while x < bounds {
    //        let (not_hit, jump) = pb.iter().fold((true, 1), |acc: (bool, usize), p| {
    //            let (hit, jump) = p.hit_and_jump(x as i64, y as i64);
    //            (acc.0 && !hit, std::cmp::max(jump, acc.1))
    //        });
    //        if not_hit {
    //            println!("Not hit! x{} y{}", x, y);
    //            not_hit_x = x;
    //            not_hit_y = y;
    //            break;
    //            // i want it to not be hit by anything
    //            //  pb1.hit() == false &&
    //        }
    //        x += jump;
    //        //println!("jumpoing {}", x);
    //    }
    //    if y % 100 == 0 {
    //        println!("on x:{},y:{} ", x, y);
    //    }
    //}

    for y in 0..bounds {
        let mut x: usize = 0;
        while x < bounds {
            let jump = pb.iter().fold(0, |acc: usize, p| {
                let jump = p.coord_jump(x as i64, y as i64);
                std::cmp::max(jump, acc)
            });
            if jump == 0 {
                println!("Not hit! x{} y{}", x, y);
                not_hit_x = x;
                not_hit_y = y;
                break;
                // i want it to not be hit by anything
                //  pb1.hit() == false &&
            }
            //println!("{} <= {}", x, jump);
            assert!(x <= jump);
            if x == jump {
                x += 1;
            } else {
                x = jump;
            }
            //println!("jumping {}", x);
        }
        if y % 100 == 0 {
            println!("on y:{} ", y);
        }
    }

    ((not_hit_x * 4000000) + not_hit_y).try_into().unwrap()
}

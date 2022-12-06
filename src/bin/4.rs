use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/4.txt");
    let input = include_str!("../../inputs/4.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 2 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 4 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

fn part_one(lines: &[&str]) -> usize {
    lines .iter().
    map(|line|{

        let pairs = line.split(",").map(|dash|{
            let (l, r) = dash.split("-").map(|side|{side.parse::<u32>().unwrap()}).collect_tuple().unwrap();
            (l, r)
        }).collect_vec();

        let left = pairs[0];
        let right = pairs[1];
        if left.0 >= right.0 && left.1 <= right.1 {
            1
        } else if right.0 >= left.0 && right.1 <= left.1 {
            1
        } else {
            0
        }
    })
    .sum()
}

fn part_two(lines: &[&str]) -> usize {
    lines .iter().
    map(|line|{

        let pairs = line.split(",").map(|dash|{
            let (l, r) = dash.split("-").map(|side|{side.parse::<u32>().unwrap()}).collect_tuple().unwrap();
            (l, r)
        }).collect_vec();

        let left = pairs[0];
        let ll = left.0;
        let lr = left.1;
        let right = pairs[1];
        let rl = right.0;
        let rr = right.1;
        if left.0 >= right.0 && left.1 <= right.1 {
            1
        } else if right.0 >= left.0 && right.1 <= left.1 {
            1
        } else if rl <= lr && ll <= rl {
            1
        } else if ll <= rr && rl <= ll {
            1
        //} else if left.0 <= right.0 && left.1 <= right.1 {
        //    1
        //} else if right.0 <= left.0 && right.1 <= left.1 {
        //    1
        } else if right.0 == left.0 || right.1 == left.1 || right.0 == left.1 || right.1 == left.0 {
            1
        } else {
            0
        }
    })
    .sum()
}

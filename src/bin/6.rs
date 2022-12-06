use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/6.txt");
    let input = include_str!("../../inputs/6.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 5 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 23 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

fn part_one(lines: &[&str]) -> usize {
    let mut first_oc = None;
    let chars: Vec<_> = lines[0].chars().collect();
    chars.as_slice().windows(4).into_iter().enumerate().for_each(|(i, p)| {
        //println!("{:?}", p);
        if first_oc == None && p[0] != p[1] && p[0] != p[2] && p[0] != p[3] && p[1] != p[2] && p[1] != p[3] && p[2] != p[3] {
            first_oc = Some(i + 4);
        }
    });
    assert!(first_oc.is_some());
    first_oc.unwrap()
}

fn part_two(lines: &[&str]) -> usize {
    let mut first_oc = None;
    let chars: Vec<_> = lines[0].chars().collect();
    chars.as_slice().windows(14).into_iter().enumerate().for_each(|(i, p)| {
        //println!("{:?}", p);
        if first_oc == None && 
            p[0] != p[1] && 
            p[0] != p[2] && 
            p[0] != p[3] && 
            p[0] != p[4] && 
            p[0] != p[5] && 
            p[0] != p[6] && 
            p[0] != p[7] && 
            p[0] != p[8] && 
            p[0] != p[9] && 
            p[0] != p[10] && 
            p[0] != p[11] && 
            p[0] != p[12] && 
            p[0] != p[13] && 
            p[1] != p[2] && 
            p[1] != p[3] && 
            p[1] != p[4] && 
            p[1] != p[5] && 
            p[1] != p[6] && 
            p[1] != p[7] && 
            p[1] != p[8] && 
            p[1] != p[9] && 
            p[1] != p[10] && 
            p[1] != p[11] && 
            p[1] != p[12] && 
            p[1] != p[13] && 
            p[2] != p[3] && 
            p[2] != p[4] && 
            p[2] != p[5] && 
            p[2] != p[6] && 
            p[2] != p[7] && 
            p[2] != p[8] && 
            p[2] != p[9] && 
            p[2] != p[10] && 
            p[2] != p[11] && 
            p[2] != p[12] && 
            p[2] != p[13] && 
            p[3] != p[4] && 
            p[3] != p[5] && 
            p[3] != p[6] && 
            p[3] != p[7] && 
            p[3] != p[8] && 
            p[3] != p[9] && 
            p[3] != p[10] && 
            p[3] != p[11] && 
            p[3] != p[12] && 
            p[3] != p[13] && 
            p[4] != p[5] && 
            p[4] != p[6] && 
            p[4] != p[7] && 
            p[4] != p[8] && 
            p[4] != p[9] && 
            p[4] != p[10] && 
            p[4] != p[11] && 
            p[4] != p[12] && 
            p[4] != p[13] && 
            p[5] != p[6] && 
            p[5] != p[7] && 
            p[5] != p[8] && 
            p[5] != p[9] && 
            p[5] != p[10] && 
            p[5] != p[11] && 
            p[5] != p[12] && 
            p[5] != p[13] && 
            p[6] != p[7] && 
            p[6] != p[8] && 
            p[6] != p[9] && 
            p[6] != p[10] && 
            p[6] != p[11] && 
            p[6] != p[12] && 
            p[6] != p[13] && 
            p[7] != p[8] && 
            p[7] != p[9] && 
            p[7] != p[10] && 
            p[7] != p[11] && 
            p[7] != p[12] && 
            p[7] != p[13] && 
            p[8] != p[9] && 
            p[8] != p[10] && 
            p[8] != p[11] && 
            p[8] != p[12] && 
            p[8] != p[13] && 
            p[9] != p[10] && 
            p[9] != p[11] && 
            p[9] != p[12] && 
            p[9] != p[13] && 
            p[10] != p[11] && 
            p[10] != p[12] && 
            p[10] != p[13] && 
            p[11] != p[12] && 
            p[11] != p[13] && 
            p[12] != p[13] 
        {
            
            first_oc = Some(i + 14);
        }
    });
    assert!(first_oc.is_some());
    first_oc.unwrap()
}

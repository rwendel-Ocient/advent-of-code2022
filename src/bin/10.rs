use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/10.txt");
    let input = include_str!("../../inputs/10.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 13140 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 0 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

fn good_cycle(c : i32) -> bool {
    let d = c -20;
    d >= 0 && (d % 40) == 0
}

fn part_one(lines: &[&str]) -> usize {
    assert!(good_cycle(20) && good_cycle(60) && good_cycle(100) && good_cycle(140));
    let mut cycle = 0;
    let mut x = 1;
    lines.iter().map(|line| {
        let mut x_reg_modifier: Option<i32> = None;
        if line.starts_with("noop") {
            // do nothing
        } else {
            // is addx
            assert!(line.starts_with("addx"));
            if let Some((_, num)) = line.split(" ").collect_tuple() {
                 x_reg_modifier = Some(num.parse::<i32>().unwrap());
            } else {
                panic!("oh no my assumptions!");
            }
        }

        if let Some(modifier) = x_reg_modifier {
            let mut result = 0;
            cycle += 1;
            if good_cycle(cycle) {
                result = cycle * x;
            }
            cycle += 1;
            if good_cycle(cycle) {
                result = cycle * x;
            }
            x += modifier;
            result

        } else {
            cycle += 1;
            if good_cycle(cycle) {
                cycle * x
            } else {
                0 as i32
            }
        }
    }).sum::<i32>() as usize

}

fn get_char(cycle: i32, x: i32) -> char {
    let mod_cycle = cycle %40;
    let hit = x-1 == mod_cycle || x  == mod_cycle || x+1 == mod_cycle;
    if hit {
        '#'
    } else {
        '.'
    }

}

fn part_two(lines: &[&str]) -> usize {
    let mut cycle = 0;
    let mut x = 1;
    let mut raster = vec![vec![' ';40]; 7];
    lines.iter().for_each(|line| {
        //print!(" {}", cycle);
        let mut x_reg_modifier: Option<i32> = None;
        if line.starts_with("noop") {
            // do nothing
        } else {
            // is addx
            assert!(line.starts_with("addx"));
            if let Some((_, num)) = line.split(" ").collect_tuple() {
                 x_reg_modifier = Some(num.parse::<i32>().unwrap());
            } else {
                panic!("oh no my assumptions!");
            }
        }

        if let Some(modifier) = x_reg_modifier {
            raster[(cycle/40) as usize][(cycle%40) as usize] = get_char(cycle, x);
            cycle += 1;
            raster[(cycle/40) as usize][(cycle%40) as usize] = get_char(cycle, x);
            cycle += 1;
            x += modifier;

        } else {
            raster[(cycle/40) as usize][(cycle%40) as usize] = get_char(cycle, x);
            cycle += 1;
        }
    });

    for y in 0..raster.len() {
        for x in 0..raster[y].len() {
            print!("{}",raster[y][x]);
        }
        println!("");
    }
    
    
    0
}

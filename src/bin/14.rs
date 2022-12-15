use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/14.txt");
    let input = include_str!("../../inputs/14.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 24 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 93 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

#[derive(Debug, Clone, Copy)]
struct PointPair {
    pub p1: (usize, usize),
    pub p2: (usize, usize),
}

impl PointPair {
    fn new(input: &[&str]) -> Self {
        assert!(input.len() == 2);
        let p1_parse: (usize, usize) = input[0]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let p2_parse: (usize, usize) = input[1]
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        PointPair {
            p1: p1_parse,
            p2: p2_parse,
        }
    }
}

impl IntoIterator for PointPair {
    type Item = (usize, usize);
    type IntoIter = PointPairIntoIterator;

    fn into_iter(self) -> Self::IntoIter {
        PointPairIntoIterator {
            index: self.p1,
            end: self.p2,
            pointPair: self,
            done: false,
        }
    }
}

pub struct PointPairIntoIterator {
    pointPair: PointPair,
    index: (usize, usize),
    end: (usize, usize),
    done: bool,
}

impl Iterator for PointPairIntoIterator {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        if self.done {
            return None;
        }
        if self.index == self.end {
            self.done = true
        }
        let result = Some(self.index);

        if self.index.0 == self.end.0 {
            if self.index.1 > self.end.1 {
                self.index.1 -= 1;
            } else if self.index.1 < self.end.1 {
                self.index.1 += 1;
            }
        } else {
            assert!(self.index.1 == self.end.1);
            if self.index.0 > self.end.0 {
                self.index.0 -= 1;
            } else if self.index.0 < self.end.0 {
                self.index.0 += 1;
            }
        }
        result
    }
}

#[derive(Debug)]
struct Line {
    pub point_array: Vec<PointPair>,
}

struct SandGrid {
    pub array: Vec<Vec<bool>>,
}

fn part_one(lines: &[&str]) -> usize {
    let line_vec = lines
        .iter()
        .map(|line| Line {
            point_array: line
                .replace(" ", "")
                .split("->")
                .collect_vec()
                .windows(2)
                .map(|l| PointPair::new(l))
                .collect_vec(),
        })
        .collect_vec();

    println!("line_vec {:?}", line_vec);
    // assume max 600 width (x)
    // assume max 200 depth (y)

    let mut grid = SandGrid {
        array: vec![vec![false; 600]; 200],
    };

    line_vec.iter().for_each(|line| {
        line.point_array.iter().for_each(|point_pair| {
            point_pair.into_iter().for_each(|(x, y)| {
                grid.array[y][x] = true;
                //println!("Wall at x{},y{}", x, y);
            });
        })
    });

    // sand comes in at 500, 0
    let mut sand_count = 0;
    loop {
        let mut sand_coord = (500, 0);
        assert!(grid.array[sand_coord.1][sand_coord.0] == false); // sand emitter should always be free
        while sand_coord.1 < 199 {
            // fall one step down
            // if one step down is blocked, diag left
            // if diag left blocked, diag right

            if grid.array[sand_coord.1 + 1][sand_coord.0] == false {
                // move down one
                sand_coord.1 += 1;
            } else if grid.array[sand_coord.1 + 1][sand_coord.0 - 1] == false {
                // down and to the left
                sand_coord.1 += 1;
                sand_coord.0 -= 1;
            } else if grid.array[sand_coord.1 + 1][sand_coord.0 + 1] == false {
                // down and to the right
                sand_coord.1 += 1;
                sand_coord.0 += 1;
            } else {
                // Stuck!
                grid.array[sand_coord.1][sand_coord.0] = true;
                //println!("Stuck at x{},y{}", sand_coord.0, sand_coord.1);
                sand_count += 1;
                break;
            }
        }

        if sand_coord.1 >= 199 {
            // we've voided
            break;
        }
    }

    sand_count
}

fn part_two(lines: &[&str]) -> usize {
    let line_vec = lines
        .iter()
        .map(|line| Line {
            point_array: line
                .replace(" ", "")
                .split("->")
                .collect_vec()
                .windows(2)
                .map(|l| PointPair::new(l))
                .collect_vec(),
        })
        .collect_vec();

    println!("line_vec {:?}", line_vec);
    // assume max 600 width (x)
    // assume max 200 depth (y)

    let mut grid = SandGrid {
        array: vec![vec![false; 800]; 175],
    };

    let mut max_height = 0;

    line_vec.iter().for_each(|line| {
        line.point_array.iter().for_each(|point_pair| {
            point_pair.into_iter().for_each(|(x, y)| {
                grid.array[y][x] = true;
                max_height = std::cmp::max(y, max_height);
                //println!("Wall at x{},y{}", x, y);
            });
        })
    });

    let floor = max_height + 2;
    println!("max height {}", floor);
    for x in 0..800 {
        grid.array[floor][x] = true
    }

    // sand comes in at 500, 0
    let mut sand_count = 0;
    loop {
        let mut sand_coord = (500, 0);
        if grid.array[sand_coord.1][sand_coord.0] == true {
            break;
        }
        while sand_coord.1 < 199 {
            // fall one step down
            // if one step down is blocked, diag left
            // if diag left blocked, diag right

            if grid.array[sand_coord.1 + 1][sand_coord.0] == false {
                // move down one
                sand_coord.1 += 1;
            } else if grid.array[sand_coord.1 + 1][sand_coord.0 - 1] == false {
                // down and to the left
                sand_coord.1 += 1;
                sand_coord.0 -= 1;
            } else if grid.array[sand_coord.1 + 1][sand_coord.0 + 1] == false {
                // down and to the right
                sand_coord.1 += 1;
                sand_coord.0 += 1;
            } else {
                // Stuck!
                grid.array[sand_coord.1][sand_coord.0] = true;
                //println!("Stuck at x{},y{}", sand_coord.0, sand_coord.1);
                sand_count += 1;
                break;
            }
        }
    }

    sand_count
}

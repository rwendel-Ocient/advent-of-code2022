use itertools::Itertools;
use std::fmt;
fn main() {
    let sample = include_str!("../../samples/8.txt");
    let input = include_str!("../../inputs/8.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 21 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 8 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

struct Vis {
    height: u32,
    fromTop: bool,
    fromLeft: bool,
    fromRight: bool,
    fromBottom: bool
}

impl Vis {
    fn is_vis(&self) -> usize {
        if self.fromTop || self.fromLeft || self.fromRight || self.fromBottom {
            1
        } else {
            0
        }

    }
}

impl fmt::Debug for Vis {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.is_vis())
    }
}

struct Vis2 {
    height: u32,
    see_left: usize,
    see_right: usize,
    see_up: usize,
    see_down: usize
}

impl Vis2 {
    fn score(&self) -> usize {
        self.see_left * self.see_right * self.see_up * self.see_down
    }
}

impl fmt::Debug for Vis2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} {} {}", self.see_left, self.see_right, self.see_up, self.see_down, self.score())
    }
}

fn part_one(lines: &[&str]) -> usize {
    let mut forest: Vec<Vec<Vis>> = vec![vec![]];

    lines.iter().for_each(|line|{
        let current = forest.last_mut().unwrap();
        line.chars().for_each(move |c| {
            current.push( Vis{ 
                height: String::from(c).parse::<u32>().unwrap(), fromTop: false, fromRight: false, fromBottom: false, fromLeft: false
            });
        });
        forest.push(vec![]);
    });
    forest.pop();

    // From top -> bottom
    for x in 0..forest[0].len() {
        let mut first = true;
        let mut tallestSoFar = 0;
        for y in 0..forest.len() {
            let t = &mut forest[y][x];
            if first {
                t.fromTop = true;
                tallestSoFar = t.height;
                first = false;
            } else {
                if tallestSoFar >= t.height {
                    t.fromTop = false;
                } else {
                    t.fromTop = true;
                    tallestSoFar = t.height
                }
            }

        }
    }

    // From bottom -> top 
    for x in 0..forest[0].len() {
        let mut first = true;
        let mut tallestSoFar = 0;
        for y in (0..forest.len()).rev() {
            let t = &mut forest[y][x];
            if first {
                t.fromBottom = true;
                tallestSoFar = t.height;
                first = false;
            } else {
                if tallestSoFar >= t.height {
                    t.fromBottom = false;
                } else {
                    t.fromBottom = true;
                    tallestSoFar = t.height
                }
            }

        }
    }

    // From left -> right
    for y in 0..forest.len() {
        let mut first = true;
        let mut tallestSoFar = 0;
        for x in 0..forest[0].len() {
            let t = &mut forest[y][x];
            if first {
                t.fromLeft = true;
                tallestSoFar = t.height;
                first = false;
            } else {
                if tallestSoFar >= t.height {
                    t.fromLeft = false;
                } else {
                    t.fromLeft = true;
                    tallestSoFar = t.height
                }
            }

        }
    }

    // From right -> Left
    for y in 0..forest.len() {
        let mut first = true;
        let mut tallestSoFar = 0;
        for x in (0..forest[0].len()).rev() {
            let t = &mut forest[y][x];
            if first {
                t.fromRight = true;
                tallestSoFar = t.height;
                first = false;
            } else {
                if tallestSoFar >= t.height {
                    t.fromRight = false;
                } else {
                    t.fromRight = true;
                    tallestSoFar = t.height
                }
            }

        }
    }
    println!("{:?}", forest);

    forest.iter().map(|line| {
        line.iter().map(|t|t.is_vis()).sum::<usize>()
    }).sum()
}

fn part_two(lines: &[&str]) -> usize {
    let mut forest: Vec<Vec<Vis2>> = vec![vec![]];

    lines.iter().for_each(|line|{
        let current = forest.last_mut().unwrap();
        line.chars().for_each(move |c| {
            current.push( Vis2{ 
                height: String::from(c).parse::<u32>().unwrap(), see_down: 0, see_left: 0, see_right: 0, see_up: 0
            });
        });
        forest.push(vec![]);
    });
    forest.pop();

    let y_len = forest.len();
    let x_len = forest[0].len();

    for outer_y in 0..y_len {
        for outer_x in 0..x_len {
            // look right
            for inner_x in outer_x+1.. x_len {
                let inner_y = outer_y;
                if forest[inner_y][inner_x].height < forest[outer_y][outer_x].height {
                    forest[outer_y][outer_x].see_right += 1;
                } else {
                    //if forest[inner_y][inner_x].height == forest[outer_y][outer_x].height {
                        forest[outer_y][outer_x].see_right += 1;
                    //}
                    break;
                }

            }
            // look left
            for inner_x in (0.. outer_x).rev() {
                let inner_y = outer_y;
                if forest[inner_y][inner_x].height < forest[outer_y][outer_x].height {
                    forest[outer_y][outer_x].see_left += 1;
                } else {
                    //if forest[inner_y][inner_x].height == forest[outer_y][outer_x].height {
                        forest[outer_y][outer_x].see_left += 1;
                    //}
                    break;
                }

            }

            // look down
            for inner_y in outer_y+1..y_len {
                let inner_x = outer_x;
                if forest[inner_y][inner_x].height < forest[outer_y][outer_x].height {
                    forest[outer_y][outer_x].see_down += 1;
                } else {
                    //if forest[inner_y][inner_x].height == forest[outer_y][outer_x].height {
                        forest[outer_y][outer_x].see_down += 1;
                    //}
                    break;
                }

            }

            // look up
            for inner_y in (0.. outer_y).rev() {
                let inner_x = outer_x;
                if forest[inner_y][inner_x].height < forest[outer_y][outer_x].height {
                    forest[outer_y][outer_x].see_up += 1;
                } else {
                    //if forest[inner_y][inner_x].height == forest[outer_y][outer_x].height {
                        forest[outer_y][outer_x].see_up += 1;
                    //}
                    break;
                }

            }
        }
    }
    println!("{:?}", forest);

    forest.iter().map(|line| {
        line.iter().map(|t|t.score()).max().unwrap()
    }).max().unwrap()
}

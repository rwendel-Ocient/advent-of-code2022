
fn main() {
    let sample = include_str!("../../samples/12.txt");
    let input = include_str!("../../inputs/12.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 31 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 29 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

struct Height {
    pub height: usize,
    pub coord: (i32, i32)
}
impl Height {
    fn new(h: usize, coord: (i32, i32)) -> Self {
        Height {
            height: h,
            coord: coord,
        }
    }
    fn eq(&self, target: &(i32, i32)) -> bool {
        self.coord.0 == target.0 && self.coord.1 == target.1
    }
}

struct Map {
    pub array: Vec<Vec<Box<Height>>>,
    pub start: (i32, i32),
    pub target: (i32, i32),
    pub width: i32,
    pub height: i32,
    pub start_array: Vec<(i32, i32)>
}

impl Map {
    fn consume(lines: &[&str]) -> Self {
        let mut array = Vec::new();
        let mut start = (0 as i32,0 as i32);
        let mut end = (0 as i32,0 as i32);
        let mut start_array = Vec::new();
        lines.iter().enumerate().for_each(|(y,line)|{
            array.push(vec![]);
            line.chars().enumerate().for_each(|(x,c)| {
                //println!("{},{}", x,y);
                
                let to_push = match c {
                    'S' => {
                        start = (x as i32,y as i32);
                        start_array.push(start.clone());
                        Box::new(Height::new(0, (x as i32,y as i32)))
                    }
                    'a' => {
                        let start2 = (x as i32,y as i32);
                        start_array.push(start2.clone());
                        Box::new(Height::new((c as usize) - ('a' as usize), (x as i32,y as i32)))
                    }
                    'E' => {
                        end = (x as i32,y as i32);
                        Box::new(Height::new(25, (x as i32,y as i32)))
                    }
                    _ => {
                        Box::new(Height::new((c as usize) - ('a' as usize), (x as i32,y as i32)))
                    }
                };
                array.last_mut().unwrap().push(to_push);
            });
        });
        let height = array.len() as i32;
        let width = array[0].len() as i32;

        println!("Start at {:?} Target is at {:?}, with width: {} and height: {}", start, end, width, height);

        Map {
            array: array,
            start: start,
            target: end,
            width: width,
            height: height,
            start_array: start_array,
        }
    }

    fn at(&self, (x,y): &(i32, i32)) -> &Height {
        &self.array[*y as usize][*x as usize]
    }

    fn valid_neighbors(&self, working: &Height) -> [Option<&Height>; 4] {
        let x = working.coord.0;
        let y = working.coord.1;

        let mut ret: [Option<&Height>; 4] = [None; 4];
        
        if x - 1 >= 0 {
            let other = self.at(&(x - 1, y));
            if other.height <= (working.height+1) {
                ret[0] = Some(other);
            }
        } 

        if x+1 < self.width {
            let other  = self.at(&(x+1, y));
            if other.height <= (working.height+1) {
                ret[1] = Some(other);
            }
        } 

        if y - 1 >= 0 {
            let other = self.at(&(x, y-1));
            if other.height <= (working.height+1) {
                ret[2] = Some(other);
            }
        } 

        if y+1 < self.height {
            let other = self.at(&(x, y+1));
            if other.height <= (working.height+1) {
                ret[3] = Some(other);
            }
        } 

        ret
    }


    fn bfs_to_target(&mut self, start: (i32, i32)) -> usize {
        let mut stack = Vec::new();

        let mut seen_bullshit: Vec<Vec<usize>> = vec![vec![std::usize::MAX; self.width as usize]; self.height as usize];

        // got damn rust and its immutibility shit
        seen_bullshit[self.start.1 as usize][self.start.0 as usize] = 0;

        stack.push((self.at(&start),0));
        let mut min_path = std::usize::MAX;
        while stack.len() > 0 {
            let (working_node, num_steps) = stack.pop().unwrap();

            // did we find the goal? 
            if working_node.eq(&self.target) {
                min_path = std::cmp::min(min_path, num_steps);
                //println!("Found target with x steps: {} ", num_steps);
            } else {
                // lol we didn't find the goal so keep looking
                let neigh = self.valid_neighbors(working_node); 
                for i in 0..neigh.len(){
                    let h = neigh[i];
                    match h {
                        Some(hh) => {
                            if num_steps+1 < seen_bullshit[hh.coord.1 as usize][hh.coord.0 as usize] {
                                //println!("Found valid neighbor {} {}, next step {}", hh.coord.0, hh.coord.1, num_steps+1);
                                seen_bullshit[hh.coord.1 as usize][hh.coord.0 as usize] = num_steps+1;
                                stack.push((hh, num_steps+1));
                            }
                        }
                        None => {}
                    }

                }
            }

        }
        min_path

    }
}

fn part_one(lines: &[&str]) -> usize {
    assert!(('z' as usize) - ('a' as usize) == 25);
    let mut map = Map::consume(lines);

    map.bfs_to_target(map.start.clone())
}

fn part_two(lines: &[&str]) -> usize {
    assert!(('z' as usize) - ('a' as usize) == 25);
    let mut map = Map::consume(lines);
    map.start_array.clone().iter().map(|s| {
        let res = map.bfs_to_target(s.clone());
        println!("was returned: {}", res);
        res
    }).min().unwrap()
}

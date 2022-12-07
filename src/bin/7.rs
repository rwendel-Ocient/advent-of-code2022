use std::vec;

use itertools::Itertools;

fn main() {
    let sample = include_str!("../../samples/7.txt");
    let input = include_str!("../../inputs/7.txt");
    let part_one = aoc::ProblemSolution::new(&part_one, 95437 /* TODO fill in */);
    let part_two = aoc::ProblemSolution::new(&part_two, 0 /* TODO fill in */);
    aoc::run_all(part_one, part_two, sample, input, sample, input);
}

#[derive(Default)]
struct File {
    name: String,
    size: usize
}

struct Dir {
    name: String,
    parent: *mut Dir,
    sub_dirs: Vec<Dir>,
    sub_files: Vec<File>
}

impl Dir {
    fn get_size(&self) -> usize {
        let mut sum: usize = self.sub_dirs.iter().map(|d| d.get_size()).sum();
        sum += self.sub_files.iter().map(|f|f.size).sum::<usize>();
        sum
    }

    fn find_dir(&mut self, name: &str) -> &mut Dir {
        for i in 0..self.sub_dirs.len() {
            if self.sub_dirs[i].name == name {
                return &mut self.sub_dirs[i];
            }
        }
        panic!("fuck")
    }

    fn root_num_exceeding(&self) -> usize {
        self.sub_dirs.iter().map(|d|d.num_exceeding()).sum::<usize>()
    }

    fn num_exceeding(&self) -> usize {
        let other = self.sub_dirs.iter().map(|d|d.num_exceeding()).sum::<usize>();
        let mut self_exceedes = 0;
        let self_size = self.get_size();
        //println!("On dir {} that has get_size {}", self.name, self_size);
        if self_size < 100000 {
            self_exceedes = self.get_size();
        }
        other + self_exceedes
    }
    
    fn smallest_that_fits(&self, needed: usize) -> Option::<(String, usize)> {
        let others = self.sub_dirs.iter().map(|d|d.smallest_that_fits(needed)).collect_vec();
        let mut best = (String::from(""), 0);
        let mut has_best = false;
        for i in 0..others.len() {
            let x = others[i].as_ref();
            if x.is_some() {
                if has_best == false {
                    best.0 = x.unwrap().0.clone();
                    best.1 = x.unwrap().1;
                    has_best = true
                } else {
                    if x.unwrap().1 < best.1 {
                        best.0 = x.unwrap().0.clone();
                        best.1 = x.unwrap().1;
                    }
                }
            }
        }

        let self_size = self.get_size();
        println!("{} {}", self_size, self.name);
        if self_size >= needed {
            if has_best {
                if self_size < best.1 {
                    best.0 = self.name.clone();
                    best.1 = self_size;
                }
            } else {
                has_best = true;
                best.0 = self.name.clone();
                best.1 = self_size;
            }
        }

        if has_best {
            Some(best)
        } else {
            None
        }
    }
}

fn part_one(lines: &[&str]) -> usize {
    let mut root = Dir {
        name: String::from(""),
        parent: std::ptr::null_mut(),
        sub_dirs: vec![],
        sub_files: vec![]
    };
    let mut current: &mut Dir = &mut root;

    let vec_of_lines = lines.iter().skip(1).collect_vec();
    for i in 0..vec_of_lines.len(){
        let line = vec_of_lines[i];
        if line.starts_with("$ ls") {
            // Do nothing here
        } else if line.starts_with("$ cd") {
            // parse
            if let Some((_,__,dir_name)) = line.split(" ").collect_tuple() {
                if dir_name == ".." {
                    unsafe {
                        current = &mut *current.parent;
                    }
                } else {
                    current = current.find_dir(dir_name);
                }
            } else {
                panic!("more than 3 tuple");
            }
        } else {
            // Add to current dir
            if let Some((t,name)) = line.split(" ").collect_tuple() {
                if t == "dir" {

                    let parent_ptr: *mut Dir = current;
                    current.sub_dirs.push(Dir {
                        name: String::from(name),
                        parent: parent_ptr,
                        sub_dirs: vec![],
                        sub_files: vec![]
                    });
                } else {
                    current.sub_files.push(File {
                        name: String::from(name),
                        size: t.parse().unwrap()
                    });
                }
            } else {
                panic!("more than 2 tuple");
            }
        }

    }
    root.root_num_exceeding()
}

fn part_two(lines: &[&str]) -> usize {
    let mut root = Dir {
        name: String::from(""),
        parent: std::ptr::null_mut(),
        sub_dirs: vec![],
        sub_files: vec![]
    };
    let mut current: &mut Dir = &mut root;

    let vec_of_lines = lines.iter().skip(1).collect_vec();
    for i in 0..vec_of_lines.len(){
        let line = vec_of_lines[i];
        if line.starts_with("$ ls") {
            // Do nothing here
        } else if line.starts_with("$ cd") {
            // parse
            if let Some((_,__,dir_name)) = line.split(" ").collect_tuple() {
                if dir_name == ".." {
                    unsafe {
                        current = &mut *current.parent;
                    }
                } else {
                    current = current.find_dir(dir_name);
                }
            } else {
                panic!("more than 3 tuple");
            }
        } else {
            // Add to current dir
            if let Some((t,name)) = line.split(" ").collect_tuple() {
                if t == "dir" {

                    let parent_ptr: *mut Dir = current;
                    current.sub_dirs.push(Dir {
                        name: String::from(name),
                        parent: parent_ptr,
                        sub_dirs: vec![],
                        sub_files: vec![]
                    });
                } else {
                    current.sub_files.push(File {
                        name: String::from(name),
                        size: t.parse().unwrap()
                    });
                }
            } else {
                panic!("more than 2 tuple");
            }
        }

    }
    let root_size = root.get_size();
    let curr_free = 70000000 - root_size;
    let needed = 30000000 - curr_free;
    println!("Need {}", needed);
    let (name, size) = root.smallest_that_fits(needed).unwrap();
    assert!(size >= needed);
    println!("dir {} has size {}", name, size);
    0
}

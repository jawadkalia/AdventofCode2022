use std::{cell::RefCell, collections::HashMap, rc::Rc, usize};

fn main() {
    let mut day7 = Day7::new();
    let mut cwd = Rc::clone(&day7.root);
    input().iter().for_each(|line| {
        let words = line.split(' ').collect::<Vec<&str>>();
        match (words[0], words[1]) {
            ("$", "ls") => {}
            ("$", "cd") => match words[2] {
                "/" => cwd = Rc::clone(&day7.root),
                ".." => {
                    cwd = Rc::clone(cwd.parent.as_ref().unwrap());
                }
                dirname => {
                    let newdir = cwd.subdir.borrow().get(dirname).unwrap().clone();
                    cwd = newdir;
                }
            },
            ("dir", dirname) => {
                cwd.subdir.borrow_mut().insert(
                    dirname.to_string(),
                    Rc::new(Dir {
                        _name: dirname.to_string(),
                        size: RefCell::new(0),
                        parent: Some(Rc::clone(&cwd)),
                        subdir: RefCell::new(HashMap::new()),
                    }),
                );
            }
            (size, _name) => {
                *cwd.size.borrow_mut() += size.parse::<usize>().unwrap();
            }
        }
    });
    println!("Day 7 a {:?}", day7.part1());
    println!("Day 7 b {:?}", day7.part2());
}

#[derive(Debug, Default)]
struct Dir {
    _name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Dir>>,
    subdir: RefCell<HashMap<String, Rc<Dir>>>,
}

impl Dir {
    fn get_size(&self) -> usize {
        *self.size.borrow()
            + self
                .subdir
                .borrow()
                .values()
                .fold(0, |a, b| a + b.get_size())
    }
}

#[derive(Debug, Default)]
pub struct Day7 {
    root: Rc<Dir>,
}

impl Day7 {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn part1(&mut self) -> usize {
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut total = 0;

        while let Some(dir) = to_visit.pop() {
            for d in dir.subdir.borrow().values() {
                to_visit.push(Rc::clone(d));
            }
            let size = dir.get_size();
            if size <= 100_000 {
                total += size;
            }
        }
        total
    }

    fn part2(&self) -> u32 {
        let total_size = self.root.get_size();
        let free_space = 70000000 - total_size;
        let space_needed = 30000000 - free_space;
        let mut to_visit = vec![Rc::clone(&self.root)];
        let mut best = usize::MAX;

        while let Some(dir) = to_visit.pop() {
            for d in dir.subdir.borrow().values() {
                to_visit.push(Rc::clone(d));
            }
            let size = dir.get_size();
            if size >= space_needed {
                best = best.min(size);
            }
        }
        best.try_into().unwrap()
    }
}

pub fn input() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

pub fn input_test() -> Vec<String> {
    include_str!("../test.txt")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
}

#[macro_use] extern crate lazy_static;
use std::cmp::min;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent;
use advent::read::read_input;

#[derive(Debug)]
struct Step {
    name: char,
    depends_on: char,
}

impl FromStr for Step {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Step ([A-Z]) must.*before step ([A-Z])").unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let depends_on = caps.get(1).unwrap().as_str().chars().next().unwrap();
                let name = caps.get(2).unwrap().as_str().chars().next().unwrap();
                return Ok(Step {name: name, depends_on: depends_on});
            },
        }
    }
}

fn part1(deps: &HashMap<char, Vec<char>>,
         revdeps: &HashMap<char, Vec<char>>) {
    let mut out = String::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut queue: BTreeSet<char> = BTreeSet::new();
    for (key, val) in deps.iter() {
        if val.len() == 0 {
            queue.insert(*key);
        }
    }
    while !queue.is_empty() {
        let item = *queue.iter().next().unwrap();
        out.push(item);
        done.insert(item);
        if revdeps.contains_key(&item) {
            for c in revdeps.get(&item).unwrap().iter() {
                if deps.get(&c).unwrap().iter().all(|x| done.contains(x)) {
                    queue.insert(*c);
                }
            }
        }
        queue.remove(&item);
    }
    println!("Part 1: {}", out);
}

const NWORKERS:usize = 5;

struct Worker {
    item: char,
    finish_at: u32,
}
impl Worker {
    fn new() -> Self {
        Self { item: ' ', finish_at: 0 }
    }
    fn is_idle(&self) -> bool {
        self.item == ' '
    }
    fn start(&mut self, item: char, now: u32) {
        assert!(self.is_idle());
        self.item = item;
        self.finish_at = now + 61 + (item as u32 - 'A' as u32);
    }
    fn reset(&mut self) {
        self.item = ' ';
    }
}

fn part2(deps: &HashMap<char, Vec<char>>,
         revdeps: &HashMap<char, Vec<char>>) {
    let mut out = String::new();
    let mut done: HashSet<char> = HashSet::new();
    let mut queue: BTreeSet<char> = BTreeSet::new();
    for (key, val) in deps.iter() {
        if val.len() == 0 {
            queue.insert(*key);
        }
    }
    let mut time = 0u32;
    let mut workers: Vec<Worker> = Vec::with_capacity(NWORKERS);
    for _ in 0..NWORKERS {
        workers.push(Worker::new());
    }
    while !queue.is_empty() || workers.iter().any(|w| !w.is_idle()) {
        let maybeworker = workers.iter_mut().find(|w| w.is_idle());
        if !queue.is_empty() && maybeworker.is_some() {
            let worker = maybeworker.unwrap();
            let item = *queue.iter().next().unwrap();
            worker.start(item, time);
            queue.remove(&item);
            continue;
        }
        else {
            // find next worker to finish and increment time
            let nexttime = workers.iter()
                .filter(|w| !w.is_idle())
                .fold(u32::max_value(), |m, w| min(m, w.finish_at));
            let mut finished = workers.iter_mut()
                .filter(|w| !w.is_idle() && w.finish_at == nexttime)
                .collect::<Vec<_>>();
            finished.sort_unstable_by_key(|w| w.item);
            for w in finished.iter_mut() {
                out.push(w.item);
                done.insert(w.item);
                if revdeps.contains_key(&w.item) {
                    for c in revdeps.get(&w.item).unwrap().iter() {
                        if deps.get(&c).unwrap().iter().all(|x| done.contains(x)) {
                            queue.insert(*c);
                        }
                    }
                }
                w.reset();
            }
            time = nexttime;
        }
    }
    println!("Part 2: {}", time);
}

fn main() {
    let data = read_input::<Step>();
    let mut deps: HashMap<char, Vec<char>> = HashMap::new();
    let mut revdeps: HashMap<char, Vec<char>> = HashMap::new();
    for step in data {
        let item = revdeps.entry(step.depends_on).or_insert(Vec::new());
        item.push(step.name);
        let item = deps.entry(step.name).or_insert(Vec::new());
        item.push(step.depends_on);

        deps.entry(step.depends_on).or_insert(Vec::new());
        revdeps.entry(step.name).or_insert(Vec::new());
    }
    part1(&deps, &revdeps);
    part2(&deps, &revdeps);
}

#[macro_use] extern crate lazy_static;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;

#[derive(Copy, Clone)]
struct Nanobot {
    x: i64,
    y: i64,
    z: i64,
    r: i64,
}
impl Nanobot {
    fn in_range(&self, other: &Nanobot) -> bool {
        self.r >= (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }
}

impl FromStr for Nanobot {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"pos=.([-0-9]+),([-0-9]+),([-0-9]+)., r=([0-9]+)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            let z = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
            let r = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Nanobot{x:x, y:y, z:z, r:r})
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

fn main() {
    let mut data = read_input::<Nanobot>();
    data.sort_unstable_by(|a, b| b.r.cmp(&a.r));
    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<Nanobot>) {
    let bot = data[0];
    let count = data.iter()
        .filter(|b| bot.in_range(b))
        .count();
    println!("Part 1: {}", count);
}

fn part2(data: &Vec<Nanobot>) {
}

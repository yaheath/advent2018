#[macro_use] extern crate lazy_static;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;

#[derive(Debug)]
struct Claim {
    name: String,
    x: i32,
    y: i32,
    h: i32,
    w: i32,
}

impl Claim {
    fn intersect(&self, other: &Claim) -> Option<Claim> {
        if other.x + other.w <= self.x || other.x >= self.x + self.w
            || other.y + other.h <= self.y || other.y >= self.y + self.h
        {
            None
        }
        else {
            let nx = max(self.x, other.x);
            let ny = max(self.y, other.y);
            let nx2 = min(self.x + self.w, other.x + other.w);
            let ny2 = min(self.y + self.h, other.y + other.h);
            Some(Claim {
                name: format!("{} {}", self.name, other.name),
                x: nx,
                y: ny,
                w: nx2 - nx,
                h: ny2 - ny,
            })
        }
    }

    //fn area(&self) -> i32 {
    //    self.w * self.h
    //}
}

impl FromStr for Claim {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(.*?) @ (-?\d+),(-?\d+): (-?\d+)x(-?\d+)").unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let name:String = caps.get(1).unwrap().as_str().to_string();
                let x:i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let y:i32 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let w:i32 = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                let h:i32 = caps.get(5).unwrap().as_str().parse::<i32>().unwrap();
                return Ok(Claim {name: name, x: x, y: y, h: h, w: w});
            },
        }
    }
}

fn read_input() -> Vec<Claim> {
    let mut data: Vec<Claim> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let val: Claim = line.trim().parse().expect("parse error");
                data.push(val);
            },
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                break;
            },
        };
    };
    return data;
}

fn part1(data: &Vec<Claim>) {
    let mut overlaps: HashSet<String> = HashSet::new();
    let mut allkeys: HashSet<String> = HashSet::new();
    data.iter().for_each(|x| {allkeys.insert(x.name.to_string());});
    for i in 0 .. data.len() - 1 {
        for j in i + 1 .. data.len() {
            if let Some(o) = data[i].intersect(&data[j]) {
                allkeys.remove(&data[i].name);
                allkeys.remove(&data[j].name);
                for x in o.x .. o.x + o.w {
                    for y in o.y .. o.y + o.h {
                        let key = format!("{},{}", x, y);
                        overlaps.insert(key);
                    }
                }
            }
        }
    }
    println!("Part 1: {}", overlaps.len());
    println!("Part 2: {}", allkeys.iter().next().unwrap());
}

fn main() {
    let data = read_input();
    part1(&data);
}

#[macro_use] extern crate lazy_static;
use std::cmp::{max, min};
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

#[derive(Debug)]
struct Coord {
    x: i64,
    y: i64,
}

impl FromStr for Coord {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+), *(\d+)").unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let x:i64 = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y:i64 = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                return Ok(Coord {x: x, y: y});
            },
        }
    }
}

fn main() {
    let data = read_input::<Coord>();
    bothparts(&data);
}

const MARGIN:i64 = 50;

fn bothparts(data: &Vec<Coord>) {
    let min_x: i64 = data.iter().map(|c| c.x)
        .fold(100000, |acc, v| min(acc, v)) - MARGIN;
    let min_y: i64 = data.iter().map(|c| c.y)
        .fold(100000, |acc, v| min(acc, v)) - MARGIN;
    let max_x: i64 = data.iter().map(|c| c.x)
        .fold(0, |acc, v| max(acc, v)) + MARGIN;
    let max_y: i64 = data.iter().map(|c| c.y)
        .fold(0, |acc, v| max(acc, v)) + MARGIN;
    let mut grid = Grid::new(min_x, min_y, max_x, max_y, -1);
    let mut td_grid = Grid::new(min_x, min_y, max_x, max_y, -1);
    for x in min_x .. max_x+1 {
        for y in min_y .. max_y+1 {
            let mut mindex = -1;
            let mut mindist = -1;
            let mut totaldist = 0;
            for (idx, point) in data.iter().enumerate() {
                let dist = (point.x - x).abs() + (point.y - y).abs();
                totaldist += dist;
                if mindist == -1 || mindist > dist {
                    mindist = dist;
                    mindex = idx as i64;
                }
                else if mindist == dist {
                    mindex = -1;
                }
            }
            grid.set(x, y, mindex);
            td_grid.set(x, y, totaldist);
        }
    }

    let mut counts = vec![0i64; data.len()];
    for x in min_x .. max_x+1 {
        let val = grid.get(x, min_y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
        let val = grid.get(x, max_y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
    }
    for y in min_y+1 .. max_y {
        let val = grid.get(min_x, y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
        let val = grid.get(max_x, y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
    }
    for x in min_x+1 .. max_x {
        for y in min_y+1 .. max_y {
            let val = grid.get(x, y);
            if val >= 0 && counts[val as usize] != -1 {
                counts[val as usize] += 1;
            }
        }
    }
    let mut maxarea = 0;
    for c in counts.iter() {
        if *c >= 0 && maxarea < *c {
            maxarea = *c;
        }
    }
    println!("Part 1: {}", maxarea);

    let mut region = 0;
    for d in td_grid.iter() {
        if *d < 10000 {
            region += 1;
        }
    }
    println!("Part 2: {}", region);
}


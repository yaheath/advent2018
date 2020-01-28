#[macro_use] extern crate lazy_static;
use std::cmp::{max, min};
use std::slice::Iter;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent;

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
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
                let x:i32 = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y:i32 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                return Ok(Coord {x: x, y: y});
            },
        }
    }
}

struct Grid {
    min_x: i32,
    min_y: i32,
    x_size: usize,
    y_size: usize,
    data: Vec<i32>,
}

impl Grid {
    pub fn new(min_x:i32, min_y:i32, max_x:i32, max_y:i32) -> Grid {
        let x_size = (max_x - min_x + 1) as usize;
        let y_size = (max_y - min_y + 1) as usize;
        let mut g = Grid {
            min_x: min_x,
            min_y: min_y,
            x_size: x_size,
            y_size: y_size,
            data: Vec::with_capacity(x_size * y_size),
        };
        for _ in 0..x_size * y_size {
            g.data.push(-1);
        }
        g
    }

    pub fn get(&self, x:i32, y:i32) -> i32 {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn set(&mut self, x:i32, y:i32, val:i32) {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
    }

    pub fn iter(&self) -> Iter<i32> {
        self.data.iter()
    }
}

fn main() {
    let data = advent::read_input::<Coord>();
    bothparts(&data);
}

const MARGIN:i32 = 50;

fn bothparts(data: &Vec<Coord>) {
    let min_x: i32 = data.iter().map(|c| c.x)
        .fold(100000, |acc, v| min(acc, v)) - MARGIN;
    let min_y: i32 = data.iter().map(|c| c.y)
        .fold(100000, |acc, v| min(acc, v)) - MARGIN;
    let max_x: i32 = data.iter().map(|c| c.x)
        .fold(0, |acc, v| max(acc, v)) + MARGIN;
    let max_y: i32 = data.iter().map(|c| c.y)
        .fold(0, |acc, v| max(acc, v)) + MARGIN;
    let mut grid = Grid::new(min_x, min_y, max_x, max_y);
    let mut td_grid = Grid::new(min_x, min_y, max_x, max_y);
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
                    mindex = idx as i32;
                }
                else if mindist == dist {
                    mindex = -1;
                }
            }
            grid.set(x, y, mindex);
            td_grid.set(x, y, totaldist);
        }
    }

    let mut counts = vec![0i32; data.len()];
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


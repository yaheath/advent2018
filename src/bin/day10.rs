#[macro_use] extern crate lazy_static;
use std::cmp::{max, min};
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent;
use advent::read::read_input;
use advent::grid::Grid;

#[derive(Clone, Copy)]
struct Point {
    x_loc: i32,
    y_loc: i32,
    x_vel: i32,
    y_vel: i32,
}

impl FromStr for Point {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"position=.\s*(-?\d+),\s*(-?\d+). velocity=.\s*(-?\d+),\s*(-?\d+)"
            ).unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let x_loc = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let y_loc = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let x_vel = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let y_vel = caps.get(4).unwrap().as_str().parse::<i32>().unwrap();
                return Ok(Point {
                    x_loc: x_loc, y_loc: y_loc, x_vel: x_vel, y_vel: y_vel,
                });
            },
        }
    }
}

fn main() {
    let data = read_input::<Point>();
    bothparts(&data);
}

fn bothparts(data: &Vec<Point>) {
    let mut stars: Vec<Point> = Vec::with_capacity(data.len());
    for s in data.iter() {
        stars.push(*s);
    }
    let mut area:i64 = -1;
    let mut minx:i32;
    let mut miny:i32;
    let mut maxx:i32;
    let mut maxy:i32;
    let mut elapsed:i32 = 0;
    loop {
        minx = stars[0].x_loc;
        miny = stars[0].y_loc;
        maxx = minx;
        maxy = miny;
        for s in stars.iter_mut() {
            s.x_loc += s.x_vel;
            s.y_loc += s.y_vel;
            minx = min(minx, s.x_loc);
            maxx = max(maxx, s.x_loc);
            miny = min(miny, s.y_loc);
            maxy = max(maxy, s.y_loc);
        }

        let newarea = (maxx - minx + 1) as i64 * (maxy - miny + 1) as i64;
        if area == -1 || area > newarea {
            area = newarea;
        }
        else if area < newarea {
            for s in stars.iter_mut() {
                s.x_loc -= s.x_vel;
                s.y_loc -= s.y_vel;
            }
            break;
        }
        elapsed += 1;
    }
    let mut grid = Grid::new(minx, miny, maxx, maxy, -1);
    for s in stars.iter() {
        grid.set(s.x_loc, s.y_loc, 1);
    }
    println!("Part 1:");
    for y in miny .. maxy+1 {
        for x in minx .. maxx+1 {
            print!("{}", if grid.get(x, y) == 1 {"#"} else {"."});
        }
        println!("");
    }
    println!("Part 2: {}", elapsed);
}


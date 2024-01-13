use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

#[derive(Clone, Copy)]
struct Point {
    x_loc: i64,
    y_loc: i64,
    x_vel: i64,
    y_vel: i64,
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
            None => Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let x_loc = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y_loc = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                let x_vel = caps.get(3).unwrap().as_str().parse::<i64>().unwrap();
                let y_vel = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
                Ok(Point {
                    x_loc, y_loc, x_vel, y_vel,
                })
            },
        }
    }
}

fn bothparts(data: &Vec<Point>) -> (String, i64) {
    let mut stars: Vec<Point> = Vec::with_capacity(data.len());
    for s in data.iter() {
        stars.push(*s);
    }
    let mut area:i64 = -1;
    let mut minx:i64;
    let mut miny:i64;
    let mut maxx:i64;
    let mut maxy:i64;
    let mut elapsed:i64 = 0;
    loop {
        minx = stars[0].x_loc;
        miny = stars[0].y_loc;
        maxx = minx;
        maxy = miny;
        for s in stars.iter_mut() {
            s.x_loc += s.x_vel;
            s.y_loc += s.y_vel;
            minx = minx.min(s.x_loc);
            maxx = maxx.max(s.x_loc);
            miny = miny.min(s.y_loc);
            maxy = maxy.max(s.y_loc);
        }

        let newarea = (maxx - minx + 1) * (maxy - miny + 1);
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
    let mut grid = Grid::new(minx, miny, maxx, maxy, 0);
    for s in stars.iter() {
        grid.set(s.x_loc, s.y_loc, 1);
    }
    (grid.format_str(|c| if c == 1 {"#".into()} else {".".into()}), elapsed)
}

fn main() {
    let data = read_input::<Point>();
    let (part1, part2) = bothparts(&data);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day10_test() {
        let input: Vec<Point> = test_input(include_str!("day10.testinput"));
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, String::from(
".............
..#...#..###.
..#...#...#..
..#...#...#..
..#####...#..
..#...#...#..
..#...#...#..
..#...#...#..
..#...#..###.
.............
.............
"));
        assert_eq!(part2, 3);
    }
}

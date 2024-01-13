use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

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
            let nx = self.x.max(other.x);
            let ny = self.y.max(other.y);
            let nx2 = (self.x + self.w).min(other.x + other.w);
            let ny2 = (self.y + self.h).min(other.y + other.h);
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
            static ref RE: Regex = Regex::new(r"^.(.*?) @ (-?\d+),(-?\d+): (-?\d+)x(-?\d+)").unwrap();
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

fn bothparts(data: &[Claim]) -> (usize, String) {
    let mut overlaps: HashSet<String> = HashSet::new();
    let mut allkeys: HashSet<String> = HashSet::from_iter(
        data.iter().map(|x| x.name.to_string())
    );
    data.iter()
        .tuple_combinations()
        .for_each(|(a, b)| {
            if let Some(o) = a.intersect(&b) {
                allkeys.remove(&a.name);
                allkeys.remove(&b.name);
                (o.x .. o.x + o.w)
                    .cartesian_product(o.y .. o.y + o.h)
                    .for_each(|(x, y)| {
                        let key = format!("{x},{y}");
                        overlaps.insert(key);
                    })
            }
        });
    (overlaps.len(), allkeys.into_iter().next().unwrap())
}

fn main() {
    let data: Vec<Claim> = read_input();
    let (part1, part2) = bothparts(&data);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day03_test() {
        let input: Vec<Claim> = test_input(include_str!("day03.testinput"));
        let (part1, part2) = bothparts(&input);
        assert_eq!(part1, 4);
        assert_eq!(part2, String::from("3"));
    }
}

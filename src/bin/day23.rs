use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Range;
use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

type Coord = (i64,i64,i64);

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
    fn center_and_corners(&self) -> Vec<Coord> {
        vec![
            (self.x, self.y, self.z),
            (self.x - self.r, self.y, self.z),
            (self.x + self.r, self.y, self.z),
            (self.x, self.y - self.r, self.z),
            (self.x, self.y + self.r, self.z),
            (self.x, self.y, self.z - self.r),
            (self.x, self.y, self.z + self.r),
        ]
    }
    fn intersects(&self, bbox: &BBox) -> bool {
        bbox.corners().iter().any(|(x,y,z)|
            self.r >= (self.x - x).abs() + (self.y - y).abs() + (self.z - z).abs()
        ) ||
        self.center_and_corners().iter().any(|(x,y,z)|
            bbox.x.contains(x) && bbox.y.contains(y) && bbox.z.contains(z)
        )
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

#[derive(Clone, Debug, Eq, PartialEq)]
struct BBox {
    x: Range<i64>,
    y: Range<i64>,
    z: Range<i64>,
}
impl BBox {
    fn volume(&self) -> i128 {
        (self.x.end - self.x.start) as i128 *
        (self.y.end - self.y.start) as i128 *
        (self.z.end - self.z.start) as i128
    }
    fn corners(&self) -> Vec<Coord> {
        vec![
            (self.x.start, self.y.start, self.z.start),
            (self.x.end - 1, self.y.start, self.z.start),
            (self.x.start, self.y.end - 1, self.z.start),
            (self.x.end - 1, self.y.end - 1, self.z.start),
            (self.x.start, self.y.start, self.z.end - 1),
            (self.x.end - 1, self.y.start, self.z.end - 1),
            (self.x.start, self.y.end - 1, self.z.end - 1),
            (self.x.end - 1, self.y.end - 1, self.z.end - 1),
        ]
    }
    fn split(&self) -> Vec<Self> {
        let mut ret = Vec::new();
        if self.x.end - self.x.start > 1 {
            let mid = if self.x.contains(&0) && self.x.start < 0
                {0} else {(self.x.end - self.x.start) / 2 + self.x.start};
            ret.push(BBox {
                x: self.x.start .. mid,
                y: self.y.clone(),
                z: self.z.clone(),
            });
            ret.push(BBox {
                x: mid .. self.x.end,
                y: self.y.clone(),
                z: self.z.clone(),
            });
        }
        else {
            ret.push(self.clone());
        }
        ret.iter()
            .flat_map(|b|
                if b.y.end - b.y.start > 1 {
                    let mid = if b.y.contains(&0) && b.y.start < 0
                        {0} else {(b.y.end - b.y.start) / 2 + b.y.start};
                    vec![
                        BBox {
                            x: b.x.clone(),
                            y: b.y.start .. mid,
                            z: b.z.clone(),
                        },
                        BBox {
                            x: b.x.clone(),
                            y: mid .. b.y.end,
                            z: b.z.clone(),
                        },
                    ]
                }
                else {
                    vec![b.clone()]
                }
            )
            .flat_map(|b|
                if b.z.end - b.z.start > 1 {
                    let mid = if b.z.contains(&0) && b.z.start < 0
                        {0} else {(b.z.end - b.z.start) / 2 + b.z.start};
                    vec![
                        BBox {
                            x: b.x.clone(),
                            y: b.y.clone(),
                            z: b.z.start .. mid,
                        },
                        BBox {
                            x: b.x.clone(),
                            y: b.y.clone(),
                            z: mid .. b.z.end,
                        },
                    ]
                }
                else {
                    vec![b.clone()]
                }
            )
            .collect()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    bbox: BBox,
    n_bots: usize,
    dx_from_origin: i64,
}

impl State {
    fn new(bbox: BBox, bots: &Vec<Nanobot>) -> Self {
        // this assumes the box does not contain any zero axes
        let dx = if bbox.x.start >= 0 { bbox.x.start } else { -(bbox.x.end - 1) };
        let dy = if bbox.y.start >= 0 { bbox.y.start } else { -(bbox.y.end - 1) };
        let dz = if bbox.z.start >= 0 { bbox.z.start } else { -(bbox.z.end - 1) };
        let n_bots = bots.iter().filter(|bot| bot.intersects(&bbox)).count();
        Self {
            bbox,
            n_bots,
            dx_from_origin: dx + dy + dz,
        }
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.n_bots.cmp(&other.n_bots)
            .then_with(|| other.dx_from_origin.cmp(&self.dx_from_origin))
            .then_with(|| other.bbox.volume().cmp(&self.bbox.volume()))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(data: &Vec<Nanobot>) -> usize {
    let bot = data[0];
    data.iter()
        .filter(|b| bot.in_range(b))
        .count()
}

fn part2(data: &Vec<Nanobot>) -> i64 {
    let mut xstart = data[0].x;
    let mut xend = data[0].x + 1;
    let mut ystart = data[0].y;
    let mut yend = data[0].y + 1;
    let mut zstart = data[0].z;
    let mut zend = data[0].z + 1;
    data.iter().for_each(|bot| {
        xstart = xstart.min(bot.x - bot.r);
        xend = xend.max(bot.x + bot.r + 1);
        ystart = ystart.min(bot.y - bot.r);
        yend = yend.max(bot.y + bot.r + 1);
        zstart = zstart.min(bot.z - bot.r);
        zend = zend.max(bot.z + bot.r + 1);
    });

    let bbox = BBox { x: xstart..xend, y: ystart..yend, z: zstart..zend };
    let mut heap = BinaryHeap::new();
    for b in bbox.split() {
        heap.push(State::new(b, data));
    }
    while let Some(state) = heap.pop() {
        if state.bbox.volume() == 1 {
            let dist = state.bbox.x.start.abs() + state.bbox.y.start.abs() + state.bbox.z.start.abs();
            return dist;
        }
        for b in state.bbox.split() {
            heap.push(State::new(b, data));
        }
    }
    panic!("no solution found");
}

fn main() {
    let mut data = read_input::<Nanobot>();
    data.sort_unstable_by(|a, b| b.r.cmp(&a.r));
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day23_test() {
        let input: Vec<Nanobot> = test_input(include_str!("day23.testinput"));
        assert_eq!(part1(&input), 7);
        let input: Vec<Nanobot> = test_input(include_str!("day23.testinput2"));
        assert_eq!(part2(&input), 36);
    }
}

use std::str::FromStr;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

#[derive(Clone, Copy, Debug)]
struct Coord {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

impl FromStr for Coord {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: Vec<i64> = s.split(',').map(|c| c.parse::<i64>().unwrap()).collect();
        Ok(Coord {
            x: c[0], y: c[1], z: c[2], w: c[3]
        })
    }
}

impl Coord {
    fn md(&self, other: &Coord) -> i64 {
        (other.x - self.x).abs() +
        (other.y - self.y).abs() +
        (other.z - self.z).abs() +
        (other.w - self.w).abs()
    }
}

// Constellation
struct C11n {
    coords: Vec<Coord>,
}

impl C11n {
    fn new(coord: Coord) -> Self {
        Self { coords: vec![coord] }
    }
    fn is_adjacent(&self, other: &Coord) -> bool {
        self.coords.iter().any(|c| c.md(other) <= 3)
    }
    fn insert(&mut self, other: Coord) {
        self.coords.push(other);
    }
    fn append(&mut self, mut other: Self) {
        self.coords.append(&mut other.coords);
    }
}

fn part1(input: &Vec<Coord>) -> usize {
    let mut cstns: Vec<C11n> = Vec::new();
    for coord in input {
        let matches: Vec<usize> = cstns.iter()
            .enumerate()
            .filter(|(_, cstn)| cstn.is_adjacent(&coord))
            .map(|(idx, _)| idx)
            .collect();
        if matches.len() == 0 {
            cstns.push(C11n::new(coord.clone()));
        }
        else {
            cstns[matches[0]].insert(coord.clone());
            let mut extracted = Vec::new();
            for idx in matches.iter().skip(1).rev() {
                extracted.push(
                    cstns.splice(idx..=idx, []).next().unwrap()
                );
            }
            for e in extracted.into_iter() {
                cstns[matches[0]].append(e);
            }
        }
    }
    cstns.len()
}

fn main() {
    let input = read_input::<Coord>();
    println!("Part 1: {}", part1(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day25_test() {
        let input: Vec<Coord> = test_input(
"0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
0,0,0,6
9,0,0,0
12,0,0,0
");
        assert_eq!(part1(&input), 2);

        let input: Vec<Coord> = test_input(
"-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0
");
        assert_eq!(part1(&input), 4);

        let input: Vec<Coord> = test_input(
"1,-1,0,1
2,0,-1,0
3,2,-1,0
0,0,3,1
0,0,-1,-1
2,3,-2,0
-2,2,0,0
2,-2,0,-1
1,-1,0,-1
3,2,0,2
");
        assert_eq!(part1(&input), 3);

        let input: Vec<Coord> = test_input(
"1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2
");
        assert_eq!(part1(&input), 8);

    }
}

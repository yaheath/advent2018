#[macro_use] extern crate lazy_static;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use itertools::Itertools;
use regex::Regex;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::read::read_input;

enum Input {
    Depth(i64),
    Target(Coord2D),
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref D_RE: Regex = Regex::new(r"depth: *(\d+)").unwrap();
        }
        lazy_static! {
            static ref T_RE: Regex = Regex::new(r"target: *(\d+),(\d+)").unwrap();
        }
        if let Some(caps) = D_RE.captures(s) {
            let d = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input::Depth(d))
        }
        else if let Some(caps) = T_RE.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input::Target(Coord2D::new(x, y)))
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

#[derive(Eq, PartialEq)]
enum CellType {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Clone, Copy)]
struct CaveCell {
    e_level: i64,
    g_index: i64,
}
impl CaveCell {
    fn new() -> Self { CaveCell{e_level: 0, g_index: 0} }
    fn cell_type(&self) -> CellType {
        match self.e_level % 3 {
            0 => CellType::Rocky,
            1 => CellType::Wet,
            2 => CellType::Narrow,
            _ => panic!(),
        }
    }
}

struct CaveGrid {
    cache: HashMap<Coord2D, CaveCell>,
    depth: i64,
    target: Coord2D,
}
impl CaveGrid {
    fn new(target: Coord2D, depth: i64) -> Self {
        Self{
            cache: HashMap::new(),
            target,
            depth,
        }
    }
    fn get(&mut self, point: Coord2D) -> CaveCell {
        if self.cache.contains_key(&point) { return self.cache[&point]; }
        let mut cc = CaveCell::new();
        if point.x == 0 {
            cc.g_index = 48271 * point.y;
        }
        else if point.y == 0 {
            cc.g_index = 16807 * point.x;
        }
        else if point == self.target {
            cc.g_index = 0;
        }
        else {
            let c1 = self.get(point + Coord2D::new(-1, 0));
            let c2 = self.get(point + Coord2D::new(0, -1));
            cc.g_index = c1.e_level * c2.e_level;
        }
        cc.e_level = (cc.g_index + self.depth) % 20183;
        self.cache.insert(point, cc);
        cc
    }
}

fn setup(input: &[Input]) -> (Coord2D, CaveGrid) {
    let depth;
    let target;
    if let Input::Depth(d) = input[0] {
        depth = d;
    } else {
        panic!("invalid input");
    }
    if let Input::Target(t) = input[1] {
        target = t;
    } else {
        panic!("invalid input");
    }
    let grid: CaveGrid = CaveGrid::new(target, depth);
    (target, grid)
}

fn part1(input: &[Input]) -> i64 {
    let (target, mut grid) = setup(input);
    (0 ..= target.x)
        .cartesian_product(0 ..= target.y)
        .map(|(x, y)| grid.get(Coord2D::new(x, y)).e_level % 3)
        .sum()
}

fn part2(input: &[Input]) -> i64 {
    let (target, mut grid) = setup(input);
    let mut dists: HashMap<(Coord2D, Tool), i64> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dists.insert((Coord2D::new(0, 0), Tool::Torch), 0);
    heap.push(State::initial(target));

    while let Some(state) = heap.pop() {
        if dists.contains_key(&state.key()) && state.cost > dists[&state.key()] {
            continue;
        }
        if state.point == target && state.tool == Tool::Torch {
            return state.cost;
        }
        let cell = grid.get(state.point);

        {
            let mut check_equip = |tool: Tool| {
                match tool {
                    Tool::Neither => if cell.cell_type() == CellType::Rocky { return; },
                    Tool::Climbing => if cell.cell_type() == CellType::Narrow { return; },
                    Tool::Torch => if cell.cell_type() == CellType::Wet { return; },
                }
                let next = state.equip_to(tool);
                if !dists.contains_key(&next.key()) || next.cost < dists[&next.key()] {
                    dists.insert(next.key(), next.cost);
                    heap.push(next);
                }
            };

            for t in [Tool::Neither, Tool::Climbing, Tool::Torch].iter() {
                if *t != state.tool { check_equip(*t); }
            }
        }

        {
            let mut check_move = |point: Coord2D| {
                let nextcell = grid.get(point);
                match state.tool {
                    Tool::Neither => if nextcell.cell_type() == CellType::Rocky { return; },
                    Tool::Climbing => if nextcell.cell_type() == CellType::Narrow { return; },
                    Tool::Torch => if nextcell.cell_type() == CellType::Wet { return; },
                }
                let next = state.move_to(point, target);
                if !dists.contains_key(&next.key()) || next.cost < dists[&next.key()] {
                    dists.insert(next.key(), next.cost);
                    heap.push(next);
                }
            };
            if state.point.y > 0 { check_move(Coord2D::new(state.point.x, state.point.y - 1)); }
            check_move(Coord2D::new(state.point.x + 1, state.point.y));
            if state.point.x > 0 { check_move(Coord2D::new(state.point.x - 1, state.point.y)); }
            check_move(Coord2D::new(state.point.x, state.point.y + 1));
        }
    }
    panic!("path not found");
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Tool {
    Torch = 0,
    Climbing,
    Neither,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: i64,
    h_cost: i64,
    point: Coord2D,
    tool: Tool,
}
impl State {
    fn initial(target: Coord2D) -> Self {
        State {
            cost: 0,
            h_cost: target.x + target.y,
            point: Coord2D::new(0, 0),
            tool: Tool::Torch,
        }
    }
    fn move_to(&self, point: Coord2D, target: Coord2D) -> Self {
        let d = point.mdist_to(&target);
        State {
            cost: self.cost + 1,
            h_cost: self.cost + 1 + d,
            point,
            tool: self.tool,
        }
    }
    fn equip_to(&self, tool: Tool) -> Self {
        State {
            cost: self.cost + 7,
            h_cost: self.h_cost + 7,
            point: self.point,
            tool,
        }
    }
    fn key(&self) -> (Coord2D, Tool) {
        (self.point, self.tool)
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.h_cost.cmp(&self.h_cost)
            .then_with(|| other.cost.cmp(&self.cost))
            .then_with(|| other.point.y.cmp(&self.point.y))
            .then_with(|| other.point.x.cmp(&self.point.x))
            .then_with(|| (other.tool as u8).cmp(&(self.tool as u8)))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = read_input::<Input>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day22_test() {
        let input: Vec<Input> = test_input("depth: 510\ntarget: 10,10\n");
        assert_eq!(part1(&input), 114);
        assert_eq!(part2(&input), 45);
    }
}

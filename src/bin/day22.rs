#[macro_use] extern crate lazy_static;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use regex::Regex;
use advent_lib::read::read_input;

enum Input {
    Depth(u64),
    Target((u64, u64)),
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
            let d = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            Ok(Input::Depth(d))
        }
        else if let Some(caps) = T_RE.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<u64>().unwrap();
            let y = caps.get(2).unwrap().as_str().parse::<u64>().unwrap();
            Ok(Input::Target((x, y)))
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
    e_level: u64,
    g_index: u64,
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
    cache: HashMap<(u64, u64), CaveCell>,
    depth: u64,
    target_x: u64,
    target_y: u64,
}
impl CaveGrid {
    fn new(target: (u64, u64), depth: u64) -> Self {
        Self{
            cache: HashMap::new(),
            target_x: target.0,
            target_y: target.1,
            depth: depth,
        }
    }
    fn get(&mut self, point: (u64, u64)) -> CaveCell {
        if self.cache.contains_key(&point) { return self.cache[&point]; }
        let mut cc = CaveCell::new();
        let (x, y) = point;
        if x == 0 {
            cc.g_index = 48271 * y;
        }
        else if y == 0 {
            cc.g_index = 16807 * x;
        }
        else if x == self.target_x && y == self.target_y {
            cc.g_index = 0;
        }
        else {
            let c1 = self.get((x - 1, y));
            let c2 = self.get((x, y - 1));
            cc.g_index = c1.e_level * c2.e_level;
        }
        cc.e_level = (cc.g_index + self.depth) % 20183;
        self.cache.insert(point, cc);
        cc
    }
}

fn main() {
    let data = read_input::<Input>();
    let depth;
    let target_x;
    let target_y;
    if let Input::Depth(d) = data[0] {
        depth = d;
    } else {
        panic!("invalid input");
    }
    if let Input::Target((x, y)) = data[1] {
        target_x = x;
        target_y = y;
    } else {
        panic!("invalid input");
    }
    let mut grid: CaveGrid = CaveGrid::new((target_x, target_y), depth);
    let mut total_risk = 0;
    for y in 0 ..= target_y {
        for x in 0 ..= target_x {
            let cc = grid.get((x, y));
            total_risk += cc.e_level % 3;
        }
    }
    println!("Part 1: {}", total_risk);

    let mut dists: HashMap<(u64, u64, Tool), u64> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dists.insert((0, 0, Tool::Torch), 0);
    heap.push(State::initial((target_x, target_y)));

    while let Some(state) = heap.pop() {
        if dists.contains_key(&state.key()) && state.cost > dists[&state.key()] {
            continue;
        }
        if state.point.0 == target_x && state.point.1 == target_y && state.tool == Tool::Torch {
            println!("Part 2: {}", state.cost);
            break;
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
            let mut check_move = |point: (u64, u64)| {
                let nextcell = grid.get(point);
                match state.tool {
                    Tool::Neither => if nextcell.cell_type() == CellType::Rocky { return; },
                    Tool::Climbing => if nextcell.cell_type() == CellType::Narrow { return; },
                    Tool::Torch => if nextcell.cell_type() == CellType::Wet { return; },
                }
                let next = state.move_to(point, (target_x, target_y));
                if !dists.contains_key(&next.key()) || next.cost < dists[&next.key()] {
                    dists.insert(next.key(), next.cost);
                    heap.push(next);
                }
            };
            if state.point.1 > 0 { check_move((state.point.0, state.point.1 - 1)); }
            check_move((state.point.0 + 1, state.point.1));
            if state.point.0 > 0 { check_move((state.point.0 - 1, state.point.1)); }
            check_move((state.point.0, state.point.1 + 1));
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
enum Tool {
    Torch = 0,
    Climbing,
    Neither,
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: u64,
    h_cost: u64,
    point: (u64, u64),
    tool: Tool,
}
impl State {
    fn initial(target: (u64, u64)) -> Self {
        State {
            cost: 0,
            h_cost: target.0 + target.1,
            point: (0, 0),
            tool: Tool::Torch,
        }
    }
    fn move_to(&self, point: (u64, u64), target: (u64, u64)) -> Self {
        let cx = if point.0 > target.0 { point.0 - target.0 } else { target.0 - point.0 };
        let cy = if point.1 > target.1 { point.1 - target.1 } else { target.1 - point.1 };
        State {
            cost: self.cost + 1,
            h_cost: self.cost + 1 + cx + cy,
            point: point,
            tool: self.tool,
        }
    }
    fn equip_to(&self, tool: Tool) -> Self {
        State {
            cost: self.cost + 7,
            h_cost: self.h_cost + 7,
            point: self.point,
            tool: tool,
        }
    }
    fn key(&self) -> (u64, u64, Tool) {
        (self.point.0, self.point.1, self.tool)
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
            .then_with(|| other.point.1.cmp(&self.point.1))
            .then_with(|| other.point.0.cmp(&self.point.0))
            .then_with(|| (other.tool as u8).cmp(&(self.tool as u8)))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

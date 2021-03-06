use std::cmp::{max, Ordering};
use std::collections::{BinaryHeap, HashMap};
use std::usize;
use std::vec::Vec;
extern crate advent;
use advent::read::read_input;
use advent::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapCell {
    Empty,
    Wall,
    Elf(usize),
    Goblin(usize),
}
impl MapCell {
    fn is_empty(&self) -> bool {
        match *self {
            MapCell::Empty => true,
            _ => false,
        }
    }
    fn from_unit(unit: &Unit) -> Self {
        if unit.is_elf {
            MapCell::Elf(unit.id)
        } else {
            MapCell::Goblin(unit.id)
        }
    }
}

struct Unit {
    id: usize,
    is_elf: bool,
    x: i32,
    y: i32,
    hp: i32,
    attack: i32,
}
impl Unit {
    fn new(id: usize, x: i32, y: i32, is_elf: bool, attack: i32) -> Self {
        Self {
            id: id,
            is_elf: is_elf,
            x: x,
            y: y,
            hp: 200,
            attack: attack,
        }
    }
    fn is_alive(&self) -> bool {
        self.hp > 0
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: i32,
    y: i32,
    path: Vec<(i32, i32)>,
}
impl State {
    fn initial(x: i32, y: i32) -> Self {
        State {
            cost: 0,
            x: x,
            y: y,
            path: Vec::new(),
        }
    }
    fn next_to(&self, x: i32, y: i32) -> Self {
        let mut path: Vec<(i32, i32)> = Vec::with_capacity(self.path.len() + 1);
        for p in self.path.iter() {
            path.push(*p);
        }
        path.push((x, y));
        State {
            cost: self.cost + 1,
            x: x,
            y: y,
            path: path,
        }
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
        other.cost.cmp(&self.cost)
            .then_with(|| other.y.cmp(&self.y))
            .then_with(|| other.x.cmp(&self.x))
            .then_with(|| other.path[0].1.cmp(&self.path[0].1))
            .then_with(|| other.path[0].0.cmp(&self.path[0].0))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy)]
enum Action {
    None,
    Move(i32, i32),
    Attack(usize),
    Finished,
}

struct Battle {
    grid: Grid<MapCell>,
    units: Vec<Unit>,
    elf_attack: i32,
}
impl Battle {
    fn new(input: &Vec<String>, elf_attack: i32) -> Self {
        let width = input.iter().map(|s| s.len()).fold(0, |maxw, w| max(w, maxw)) as i32;
        let height = input.len() as i32;
        let mut y = 0i32;
        let mut inst = Self {
            grid: Grid::new(0, 0, width-1, height-1, MapCell::Empty),
            units: Vec::new(),
            elf_attack: elf_attack,
        };
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i32;
                match c {
                    '#' => inst.put_wall(x, y),
                    'E' => inst.put_elf(x, y),
                    'G' => inst.put_goblin(x, y),
                    _ => (),
                };
            }
            y += 1;
        }
        inst
    }
    fn put_wall(&mut self, x: i32, y: i32) {
        self.grid.set(x, y, MapCell::Wall);
    }
    fn put_elf(&mut self, x: i32, y: i32) {
        self.grid.set(x, y, MapCell::Elf(self.units.len()));
        self.units.push(Unit::new(self.units.len(), x, y, true, self.elf_attack));
    }
    fn put_goblin(&mut self, x: i32, y: i32) {
        self.grid.set(x, y, MapCell::Goblin(self.units.len()));
        self.units.push(Unit::new(self.units.len(), x, y, false, 3));
    }

    fn step(&mut self) -> bool {
        let mut unit_ids: Vec<usize> =
            self.units.iter().filter(|u| u.is_alive()).map(|u| u.id).collect();
        unit_ids.sort_unstable_by(|ia, ib| {
            let a = &self.units[*ia];
            let b = &self.units[*ib];
            a.y.cmp(&b.y).then_with(|| a.x.cmp(&b.x))
        });
        for id in unit_ids.iter() {
            { // limit the scope of the immutable `unit` borrow
                let unit = &self.units[*id];
                if !unit.is_alive() { continue; }
                assert_eq!(MapCell::from_unit(unit), self.grid.get(unit.x, unit.y));
            }

            match self.determine_action(*id) {
                Action::Finished => return false,
                Action::Move(x, y) => {
                    self.move_to(*id, x, y);
                    if let Action::Attack(enemy_id) = self.determine_attack(*id) {
                        self.attack(*id, enemy_id);
                    }
                },
                Action::Attack(enemy_id) => self.attack(*id, enemy_id),
                Action::None => (),
            }
        }
        true
    }

    fn determine_action(&self, unit_id: usize) -> Action {
        if let Action::Attack(enemy_id) = self.determine_attack(unit_id) {
            return Action::Attack(enemy_id);
        }
        let unit = &self.units[unit_id];

        // See if there's an enemy we can move toward, using Dijkstra
        let mut dists: HashMap<(i32, i32), usize> = HashMap::new();
        let mut heap = BinaryHeap::new();
        let mut mindist = usize::MAX;
        let mut candidates = BinaryHeap::new();
        dists.insert((unit.x, unit.y), 0);
        heap.push(State::initial(unit.x, unit.y));

        while let Some(state) = heap.pop() {
            if dists.contains_key(&(state.x, state.y)) && state.cost > dists[&(state.x, state.y)] {
                continue;
            }
            if state.cost > mindist {
                continue;
            }

            let mut check = |x: i32, y: i32| {
                if let Some(_) = self.is_enemy(unit, x, y) {
                    if state.cost <= mindist {
                        candidates.push(state.clone());
                        mindist = state.cost;
                    }
                }
                else if self.grid.get(x, y).is_empty() {
                    let next = state.next_to(x, y);
                    if !dists.contains_key(&(x, y)) || next.cost < dists[&(x, y)] {
                        dists.insert((x, y), next.cost);
                        heap.push(next);
                    }
                }
            };
            check(state.x, state.y - 1);
            check(state.x - 1, state.y);
            check(state.x + 1, state.y);
            check(state.x, state.y + 1);
        }
        if let Some(to) = candidates.pop() {
            return Action::Move(to.path[0].0, to.path[0].1);
        }
        if self.units.iter().filter(|u| u.is_elf != unit.is_elf).all(|u| !u.is_alive()) {
            return Action::Finished;
        }
        return Action::None
    }

    // see if there's an adjacent unit we can attack
    fn determine_attack(&self, unit_id: usize) -> Action {
        let unit = &self.units[unit_id];

        let mut in_range: Vec<&Unit> = Vec::with_capacity(4);
        if let Some(enemy) = self.is_enemy(unit, unit.x, unit.y - 1) {
            in_range.push(enemy);
        }
        if let Some(enemy) = self.is_enemy(unit, unit.x - 1, unit.y) {
            in_range.push(enemy);
        }
        if let Some(enemy) = self.is_enemy(unit, unit.x + 1, unit.y) {
            in_range.push(enemy);
        }
        if let Some(enemy) = self.is_enemy(unit, unit.x, unit.y + 1) {
            in_range.push(enemy);
        }
        if in_range.len() > 0 {
            in_range.sort_by(|a, b| a.hp.cmp(&b.hp));
            Action::Attack(in_range[0].id)
        }
        else {
            Action::None
        }
    }

    fn is_enemy(&self, unit: &Unit, x: i32, y:i32) -> Option<&Unit> {
        if unit.is_elf {
            match self.grid.get(x, y) {
                MapCell::Goblin(id) => Some(&self.units[id]),
                _ => None,
            }
        } else {
            match self.grid.get(x, y) {
                MapCell::Elf(id) => Some(&self.units[id]),
                _ => None,
            }
        }
    }

    fn move_to(&mut self, unit_id: usize, x: i32, y: i32) {
        let unit = &mut self.units[unit_id];
        assert_eq!(self.grid.get(x, y), MapCell::Empty);
        assert_eq!(MapCell::from_unit(unit), self.grid.get(unit.x, unit.y));
        self.grid.set(unit.x, unit.y, MapCell::Empty);
        unit.x = x;
        unit.y = y;
        self.grid.set(unit.x, unit.y, MapCell::from_unit(unit));
    }

    fn attack(&mut self, unit_id: usize, enemy_id: usize) {
        assert_ne!(unit_id, enemy_id);
        let split_at = max(unit_id, enemy_id);
        let (left, right) = self.units.split_at_mut(split_at);
        let unit: &mut Unit;
        let enemy: &mut Unit;
        if unit_id < enemy_id {
            unit = &mut left[unit_id];
            enemy = &mut right[0];
        } else {
            enemy = &mut left[enemy_id];
            unit = &mut right[0];
        }
        enemy.hp -= unit.attack;
        if enemy.hp <= 0 {
            self.grid.set(enemy.x, enemy.y, MapCell::Empty);
        }
    }
}

fn main() {
    let data = read_input::<String>();
    part1(&data);
    part2(&data);
}

fn part1(data: &Vec<String>) {
    let mut battle = Battle::new(&data, 3);
    let mut turns = 0i32;

    while battle.step() {
        turns += 1;
    }
    let sum = battle.units.iter().filter(|u| u.is_alive()).fold(0, |sum, u| sum + u.hp);
    println!("Part 1: {}", sum * turns);
}

fn part2(data: &Vec<String>) {
    let mut lower = 3i32;
    let mut upper = 200i32;
    let mut lastwin = 0i32;
    while lower + 1 < upper {
        let mid = lower + (upper - lower) / 2;
        //println!("testing {}", mid);
        let mut battle = Battle::new(&data, mid);
        let mut turns = 0;
        while battle.step() {
            turns += 1;
        }
        let failed = battle.units.iter().any(|u| u.is_elf && !u.is_alive());
        if failed {
            lower = mid;
        } else {
            upper = mid;
            let sum = battle.units.iter().filter(|u| u.is_alive()).fold(0, |sum, u| sum + u.hp);
            lastwin = turns * sum;
        }
    }
    println!("Part 2: {}", lastwin);
}

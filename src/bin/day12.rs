#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent;

enum InputItem {
    InitialState(Vec<bool>),
    MapItem(u32, bool),
    None,
}

impl FromStr for InputItem {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_INIT: Regex = Regex::new(
                r"initial state: ([#.]+)",
            ).unwrap();
        }
        lazy_static! {
            static ref RE_BLANK: Regex = Regex::new(
                r"^\s*$",
            ).unwrap();
        }
        lazy_static! {
            static ref RE_MAP: Regex = Regex::new(
                r"([#.]+) => ([#.])",
            ).unwrap();
        }
        if let Some(caps) = RE_INIT.captures(s) {
            let isstr = caps.get(1).unwrap().as_str();
            let mut state:Vec<bool> = Vec::with_capacity(isstr.len());
            for c in isstr.chars() {
                state.push(c == '#');
            }
            return Ok(InputItem::InitialState(state));
        }
        if let Some(_) = RE_BLANK.captures(s) {
            return Ok(InputItem::None);
        }
        if let Some(caps) = RE_MAP.captures(s) {
            let sstr = caps.get(1).unwrap().as_str();
            let ostr = caps.get(2).unwrap().as_str();
            let mut k = 0u32;
            for c in sstr.chars() {
                k <<= 1;
                if c == '#' {
                    k |= 1;
                }
            }
            return Ok(
                InputItem::MapItem(k, ostr.chars().next().unwrap() == '#')
            );
        }
        Err("invalid input line".to_string())
    }
}

fn main() {
    let data = advent::read_input::<InputItem>();
    let mut map: HashMap<u32, bool> = HashMap::with_capacity(data.len() - 2);
    let mut initial: Option<&Vec<bool>> = None;
    for i in 0..32 {
        map.insert(i, false);
    }
    for d in data.iter() {
        match d {
            InputItem::InitialState(v) => {initial = Some(&v);},
            InputItem::MapItem(k, v) => {map.insert(*k, *v);},
            InputItem::None => (),
        }
    }
    part1(&map, &initial.unwrap());
    part2(&map, &initial.unwrap());
}

fn stringify(pots: &advent::NumberLine<bool>) -> String {
    pots.enumerate().map(|v| if v.1 { '#' } else { '.' }).collect::<String>()
}

fn part1(map: &HashMap<u32, bool>, initial: &Vec<bool>) {
    let mut pots = advent::NumberLine::<bool>::from_initial(initial, false);
    for _ in 0..20 {
        step(&mut pots, map);
    }
    let mut sum = 0i64;
    for idx in pots.start_index() .. pots.end_index() {
        if pots[idx] { sum += idx; }
    }
    println!("Part 1: {}", sum);
}

fn part2(map: &HashMap<u32, bool>, initial: &Vec<bool>) {
    let mut pots = advent::NumberLine::<bool>::from_initial(initial, false);
    let mut states: HashMap<String, (i64, usize)> = HashMap::new();
    let mut gen = 0usize;
    loop {
        let s = stringify(&pots);
        let startidx = pots.enumerate().next().unwrap().0;
        if states.contains_key(&s) {
            let prevstartidx = states[&s].0;
            let prevgen = states[&s].1;
            if prevstartidx == startidx {
                todo!("stable/metastable");
            }
            else if prevgen + 1 != gen {
                todo!("more than one step between repeat");
            }
            else {
                //println!("gen={} prevgen={} startidx={} prevstartidx={}",
                //         gen, prevgen, startidx, prevstartidx);
                let shiftpergen = startidx - prevstartidx;
                let fiftybil = 50000000000usize;
                let remain = fiftybil - gen;
                let shift = shiftpergen * (remain as i64);
                let mut sum = 0i64;
                for (idx, val) in pots.enumerate() {
                    if val { sum += idx + shift; }
                }
                println!("Part 2: {}", sum);
            }
            break;
        }
        states.insert(s, (startidx, gen));
        step(&mut pots, map);
        gen += 1;
    }
}

fn step(pots: &mut advent::NumberLine<bool>, map: &HashMap<u32, bool>) {
    let oldpots = pots.clone();
    for idx in oldpots.start_index() - 2 .. oldpots.end_index() + 2 {
        let mut val: u32 = 0;
        for p in idx - 2 .. idx + 3 {
            val <<= 1;
            if oldpots[p] { val |= 1; }
        }
        pots[idx] = map[&val];
    }
}

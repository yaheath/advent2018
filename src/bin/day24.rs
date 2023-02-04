#[macro_use] extern crate lazy_static;
use std::collections::HashSet;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;

#[derive(Clone)]
struct Group {
    n_units: usize,
    hp: i64,
    dmg: i64,
    init: i64,
    attack: String,
    weak_to: HashSet<String>,
    immune_to: HashSet<String>,
}
impl Group {
}

impl FromStr for Group {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) units.* (\d+) hit points( \(.*\))? with.*does (\d+) (\w+) .*ive (\d+)").unwrap();
        }
        lazy_static! {
            static ref A_RE: Regex = Regex::new(r"\((\w+) to (\w+(?:, \w+)*)(?:; (\w+) to (\w+(?:, \w+)*))?\)").unwrap();
        }
        if let Some(caps) = RE.captures(s) {
            let weak_to: HashSet<String> = HashSet::new();
            let immune_to: HashSet<String> = HashSet::new();
            let n = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let hp = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            if let Some(attr_cap) = caps.get(3) {
                if let Some(acaps) = A_RE.captures(attr_cap.as_str()) {
                    println!("{:?}", acaps);
                }
                else {
                    return Err(format!("invalid attr: {}", s))
                }
            }
            let d = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let a = caps.get(5).unwrap().as_str().to_string();
            let i = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Group {
                n_units: n,
                hp: hp,
                dmg: d,
                attack: a,
                init: i,
                weak_to: weak_to,
                immune_to: immune_to,
            })
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

fn main() {
    let mut data = read_input::<Group>();
}


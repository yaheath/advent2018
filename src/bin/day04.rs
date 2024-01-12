use std::collections::HashMap;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use advent_lib::read::read_input;

struct Guard {
    id: i32,
    log: HashMap<String, [bool; 60]>,
}

impl Guard {
    fn time_asleep(&self) -> usize {
        self.log.values()
            .map(|sleep| sleep.iter().filter(|b| **b).count())
            .sum()
    }
}

fn setup(input: &[String]) -> HashMap<i32, Guard> {
    let mut cur_guard_id: i32 = -1;
    let mut sleeping: bool = false;
    let mut sleepstart: usize = 0;
    let mut guards: HashMap<i32, Guard> = HashMap::new();

    for row in input.iter().sorted_unstable() {
        lazy_static! {
            static ref LINE_RE: Regex = Regex::new(r"\[([-0-9]+) (\d\d):(\d\d)\] (.*)\n?").unwrap();
        }
        lazy_static! {
            static ref ACTION_RE: Regex = Regex::new(r"(\w+) (?:\#(\d+))?").unwrap();
        }
        let date: String;
        let hour: usize;
        let minute: usize;
        let action: String;
        match LINE_RE.captures(row) {
            None => {
                eprintln!("invalid input: {}", row);
                continue;
            },
            Some(caps) => {
                date = caps.get(1).unwrap().as_str().to_string();
                hour = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                minute = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
                action = caps.get(4).unwrap().as_str().to_string();
            }
        }
        match ACTION_RE.captures(&action) {
            None => {
                eprintln!("invalid input: {}", row);
                continue;
            },
            Some(caps) => {
                let a = caps.get(1).unwrap().as_str();
                if a == "Guard" {
                    let id = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    if sleeping {
                        eprintln!("guard {} asleep at end of shift", cur_guard_id);
                    }
                    cur_guard_id = id;
                    sleeping = false;
                }
                else {
                    assert!(cur_guard_id != -1);
                    let cur_guard = guards.entry(cur_guard_id)
                        .or_insert(Guard { id: cur_guard_id, log: HashMap::new() });
                    if a == "falls" {
                        sleeping = true;
                        sleepstart = if hour == 0 { minute } else { 0 };
                    }
                    else if a == "wakes" {
                        if hour == 0 || hour == 1 {
                            let mut stop: usize = 60;
                            if hour == 0 {
                                stop = minute;
                            }
                            let log = cur_guard.log.entry(date).or_insert([false; 60]);

                            for i in sleepstart .. stop {
                                log[i] = true;
                            }
                        }
                        sleeping = false;
                    }
                }
            }
        }
    }
    guards
}

fn part1(guards: &HashMap<i32, Guard>) -> i32 {
    let guard = guards.values()
        .map(|g| (g, g.time_asleep()))
        .max_by_key(|(_,t)| *t)
        .map(|(g, _)| g)
        .unwrap();
    let maxminute = (0..60).map(|m|
            (m, guard.log.values().filter(|log| log[m as usize]).count())
        )
        .max_by_key(|(_, sum)| *sum)
        .map(|(m, _)| m)
        .unwrap();
    maxminute * guard.id
}

fn part2(guards: &HashMap<i32, Guard>) -> i32 {
    let (guard, minute) = guards.values()
        .map(|guard| (guard,
            (0..60).map(|m|
                (m, guard.log.values().filter(|log| log[m as usize]).count())
            )
            .max_by_key(|(_, sum)| *sum)
            .unwrap()
        ))
        .max_by_key(|(_,(_,t))| *t)
        .map(|(g, (m, _))| (g, m))
        .unwrap();
    guard.id * minute
}

fn main() {
    let input: Vec<String> = read_input();
    let guards = setup(&input);
    println!("Part 1: {}", part1(&guards));
    println!("Part 2: {}", part2(&guards));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day04_test() {
        let input: Vec<String> = test_input(include_str!("day04.testinput"));
        let guards = setup(&input);
        assert_eq!(part1(&guards), 240);
        assert_eq!(part2(&guards), 4455);
    }
}

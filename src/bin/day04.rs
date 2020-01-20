#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::collections::HashMap;
extern crate advent;

struct Guard {
    //id: i32,
    log: HashMap<String, [bool; 60]>,
}

impl Guard {
    fn time_asleep(&self) -> usize {
        let mut minutes: usize = 0;
        for (_, sleep) in self.log.iter() {
            for b in sleep.iter() {
                if *b {
                    minutes += 1;
                }
            }
        }
        minutes
    }
}

fn main() {
    let mut data = advent::read_input::<String>();
    data.sort_unstable();

    let mut cur_guard_id: i32 = -1;
    let mut sleeping: bool = false;
    let mut sleepstart: usize = 0;
    let mut guards: HashMap<i32, Guard> = HashMap::new();

    for row in data {
        let row: String = row.to_string();
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
        match LINE_RE.captures(&row) {
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
                        .or_insert(Guard { log: HashMap::new() });
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

    part1(&guards);
    part2(&guards);
}

fn part1(guards: &HashMap<i32, Guard>) {
    let mut max: usize = 0;
    let mut guard_id = 0;
    for (id, guard) in guards.iter() {
        let time = guard.time_asleep();
        if time > max {
            max = time;
            guard_id = *id;
        }
    }
    let guard = guards.get(&guard_id).unwrap();
    let mut maxminute: i32 = 0;
    let mut maxsum: i32 = -1;
    for m in 0..60 {
        let mut sum = 0;
        for (_, log) in guard.log.iter() {
            if log[m as usize] { sum += 1; }
        }
        if sum > maxsum {
            maxsum = sum;
            maxminute = m;
        }
    }
    println!("Part 1: {}", maxminute * guard_id);
}

fn part2(guards: &HashMap<i32, Guard>) {
    let mut maxguard: i32 = 0;
    let mut maxguardsum: i32 = 0;
    let mut maxguardminute: i32 = 0;
    for (id, guard) in guards.iter() {
        let mut maxminute: i32 = 0;
        let mut maxsum: i32 = -1;
        for m in 0..60 {
            let mut sum = 0;
            for (_, log) in guard.log.iter() {
                if log[m as usize] { sum += 1; }
            }
            if sum > maxsum {
                maxsum = sum;
                maxminute = m;
            }
        }
        if maxsum > maxguardsum {
            maxguardsum = maxsum;
            maxguardminute = maxminute;
            maxguard = *id;
        }
    }
    println!("Part 2: {}", maxguardminute * maxguard);
}

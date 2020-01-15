use std::collections::HashMap;
use std::io::{self, BufRead};
use std::option::Option;
use std::vec::Vec;

fn read_input() -> Vec<String> {
    let mut data: Vec<String> = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                let val: String = line.trim().to_string();
                data.push(val);
            },
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                break;
            },
        };
    };
    return data;
}

fn part1(input: &Vec<String>) {
    let mut twos: i32 = 0;
    let mut threes: i32 = 0;
    let mut chars: HashMap<char, i32> = HashMap::new();
    for label in input.iter() {
        for c in label.chars() {
            let val = chars.entry(c).or_insert(0);
            *val += 1;
        }
        let mut havetwo = false;
        let mut havethree = false;
        for (_, val) in chars.iter() {
            if *val == 2 {
                havetwo = true;
            }
            if *val == 3 {
                havethree = true;
            }
        }
        if havetwo {
            twos += 1;
        }
        if havethree {
            threes += 1;
        }
        chars.clear();
    }
    println!("Part 1: {}", twos * threes);
}

fn part2(input: &Vec<String>) {
    for i in 0..(input.len()-1) {
        for j in (i+1)..input.len() {
            match check_string_diff(&input[i], &input[j]) {
                Some(common) => {
                    println!("Part 2: {}", common);
                    return;
                },
                None => (),
            }
        }
    }
}

fn check_string_diff(s1: &String, s2: &String) -> Option<String> {
    let mut ndiff: usize = 0;
    let mut common: String = String::new();
    let mut s1_itr = s1.chars();
    for s2c in s2.chars() {
        let s1c = s1_itr.next().unwrap();
        if s1c != s2c {
            ndiff += 1;
        }
        else {
            common.push(s1c);
        }
    }
    if ndiff != 1 {
        return None;
    }
    return Some(common);
}

fn main() {
    let input: Vec<String> = read_input();
    part1(&input);
    part2(&input);
}

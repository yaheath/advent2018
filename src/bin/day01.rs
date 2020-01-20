use std::collections::HashSet;
use std::vec::Vec;
extern crate advent;

fn part1(input: &Vec<i32>) {
    let mut freq: i32 = 0;
    for n in input.iter() {
        freq += n;
    }
    println!("Part 1: {}", freq);
}

fn part2(input: &Vec<i32>) {
    let mut freq: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();
    loop {
        for n in input.iter() {
            freq += n;
            if set.contains(&freq) {
                println!("Part 2: {}", freq);
                return;
            }
            set.insert(freq);
        }
    }
}

fn main() {
    let input: Vec<i32> = advent::read_input::<i32>();
    part1(&input);
    part2(&input);
}

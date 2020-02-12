use std::vec::Vec;
extern crate advent;
use advent::read::read_input;

struct State {
    scores: Vec<usize>,
    elf1: usize,
    elf2: usize,
}
impl State {
    fn new() -> Self {
        let mut scores = Vec::new();
        scores.push(3);
        scores.push(7);
        State {
            scores: scores,
            elf1: 0,
            elf2: 1,
        }
    }
    fn step(&mut self) {
        let s1 = self.scores[self.elf1];
        let s2 = self.scores[self.elf2];
        let score = s1 + s2;
        if score > 9 {
            self.scores.push(1);
        }
        self.scores.push(score % 10);
        self.elf1 = (self.elf1 + 1 + s1) % self.scores.len();
        self.elf2 = (self.elf2 + 1 + s2) % self.scores.len();
    }
}

fn main() {
    let input: Vec<String> = read_input::<String>();
    let n_recipes = input[0].parse::<usize>().unwrap();
    part1(n_recipes);
    part2(&input[0]);
}

fn part1(n_recipes: usize) {
    let mut state = State::new();
    while state.scores.len() < n_recipes + 10 {
        state.step();
    }
    print!("Part 1: ");
    for n in n_recipes .. n_recipes + 10 {
        print!("{}", state.scores[n]);
    }
    println!("");
}

fn part2(seq: &String) {
    let digits: Vec<usize> = seq.chars().map(|c| c as usize - '0' as usize).collect();
    let mut state = State::new();
    let mut curindex:usize = 0;
    loop {
        state.step();
        while curindex + digits.len() < state.scores.len() {
            if &state.scores[curindex .. curindex + digits.len()] == &digits[..] {
                println!("Part 2: {}", curindex);
                return;
            }
            curindex += 1;
        }
    }
}

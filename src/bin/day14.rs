use std::vec::Vec;
use ya_advent_lib::read::read_input;

struct State {
    scores: Vec<usize>,
    elf1: usize,
    elf2: usize,
}
impl State {
    fn new() -> Self {
        State {
            scores: vec![3, 7],
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

fn part1(input: &str) -> String {
    let n_recipes = input.parse::<usize>().unwrap();
    let mut state = State::new();
    while state.scores.len() < n_recipes + 10 {
        state.step();
    }
    (n_recipes .. n_recipes + 10).map(|n| ((state.scores[n] as u8) + b'0') as char).collect()
}

fn part2(seq: &str) -> usize {
    let digits: Vec<usize> = seq.chars().map(|c| c as usize - '0' as usize).collect();
    let mut state = State::new();
    let mut curindex:usize = 0;
    loop {
        state.step();
        while curindex + digits.len() < state.scores.len() {
            if state.scores[curindex .. curindex + digits.len()] == digits[..] {
                return curindex;
            }
            curindex += 1;
        }
    }
}

fn main() {
    let input: Vec<String> = read_input::<String>();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_test() {
        assert_eq!(part1("9"), "5158916779".to_string());
        assert_eq!(part1("5"), "0124515891".to_string());
        assert_eq!(part1("18"), "9251071085".to_string());
        assert_eq!(part1("2018"), "5941429882".to_string());
        assert_eq!(part2("51589"), 9);
        assert_eq!(part2("01245"), 5);
        assert_eq!(part2("92510"), 18);
        assert_eq!(part2("59414"), 2018);
    }
}

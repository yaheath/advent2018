#[macro_use] extern crate lazy_static;
use linked_list::{Cursor, LinkedList};
use std::cmp::max;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
extern crate advent;

struct Input {
    n_players: usize,
    max_marble: usize,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"(\d+) players.*worth (\d+)").unwrap();
        }
        match RE.captures(s) {
            None => return Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let n_players = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let max_marble = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                return Ok(Input {n_players: n_players, max_marble: max_marble});
            },
        }
    }
}

fn main() {
    let data = advent::read_input::<Input>();
    part1(&data[0]);
    part2(&data[0]);
}

fn part1(data: &Input) {
    let max_score = play_game(data.n_players, data.max_marble);
    println!("Part 1: {}", max_score);
}

fn part2(data: &Input) {
    let max_score = play_game(data.n_players, data.max_marble * 100);
    println!("Part 2: {}", max_score);
}

// Similar to cursor.seek_forward() but will automatically skip the
// None at the head/tail
fn cursor_forward(cursor: &mut Cursor<usize>, nsteps: usize) {
    for _ in 0 .. nsteps {
        let item = cursor.next();
        if item.is_none() {
            cursor.next();
        }
    }
}
fn cursor_backward(cursor: &mut Cursor<usize>, nsteps: usize) {
    for _ in 0 .. nsteps {
        let item = cursor.prev();
        if item.is_none() {
            cursor.prev();
        }
    }
}

fn play_game(n_players: usize, max_marble: usize) -> usize {
    let mut scores: Vec<usize> = vec![0; n_players];
    let mut ring: LinkedList<usize> = LinkedList::new();
    ring.push_front(0);
    let mut cursor = ring.cursor();
    let mut player = 0;
    for n in 1 .. max_marble + 1 {
        if n % 23 == 0 {
            scores[player] += n;
            cursor_backward(&mut cursor, 7);
            if cursor.peek_next().is_none() {
                cursor.next();
            }
            scores[player] += cursor.remove().unwrap();
        }
        else {
            cursor_forward(&mut cursor, 2);
            cursor.insert(n);
        }
        player = (player + 1) % n_players;
    }
    scores.iter().fold(0usize, |m, s| max(m, *s))
}

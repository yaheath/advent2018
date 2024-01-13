use std::str::FromStr;
use std::vec::Vec;
use lazy_static::lazy_static;
use linked_list::{Cursor, LinkedList};
use regex::Regex;
use ya_advent_lib::read::read_input;

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
            None => Err(format!("invalid input: {}", s)),
            Some(caps) => {
                let n_players = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let max_marble = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
                Ok(Input {n_players, max_marble})
            },
        }
    }
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
    scores.into_iter().max().unwrap()
}

fn part1(data: &Input) -> usize {
    play_game(data.n_players, data.max_marble)
}

fn part2(data: &Input) -> usize {
    play_game(data.n_players, data.max_marble * 100)
}

fn main() {
    let data = read_input::<Input>();
    println!("Part 1: {}", part1(&data[0]));
    println!("Part 2: {}", part2(&data[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day09_test() {
        let input: Vec<Input> = test_input("10 players; last marble is worth 1618 points");
        assert_eq!(part1(&input[0]), 8317);
        assert_eq!(part2(&input[0]), 74765078);
        let input: Vec<Input> = test_input("13 players; last marble is worth 7999 points");
        assert_eq!(part1(&input[0]), 146373);
        assert_eq!(part2(&input[0]), 1406506154);
        let input: Vec<Input> = test_input("17 players; last marble is worth 1104 points");
        assert_eq!(part1(&input[0]), 2764);
        assert_eq!(part2(&input[0]), 20548882);
        let input: Vec<Input> = test_input("21 players; last marble is worth 6111 points");
        assert_eq!(part1(&input[0]), 54718);
        assert_eq!(part2(&input[0]), 507583214);
        let input: Vec<Input> = test_input("30 players; last marble is worth 5807 points");
        assert_eq!(part1(&input[0]), 37305);
        assert_eq!(part2(&input[0]), 320997431);
    }
}

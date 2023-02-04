use std::slice::Iter;
use std::str::FromStr;
use std::vec::Vec;
use advent_lib::read::read_input;

struct Input {
    list: Vec<usize>,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Input {
            list: s.split_whitespace()
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<_>>(),
        })
    }
}

fn main() {
    let data = read_input::<Input>();
    part1(&(data[0].list));
    part2(&(data[0].list));
}

fn part1(input: &Vec<usize>) {
    let mut iter = input.iter();
    let result = part1_recurse(&mut iter);
    println!("Part 1: {}", result);
}

fn part1_recurse(iter: &mut Iter<usize>) -> usize {
    let n_children = *iter.next().unwrap();
    let n_metas = *iter.next().unwrap();
    let mut sum: usize = 0;
    for _ in 0..n_children {
        sum += part1_recurse(iter);
    }
    for _ in 0..n_metas {
        sum += *iter.next().unwrap();
    }
    sum
}

fn part2(input: &Vec<usize>) {
    let mut iter = input.iter();
    let result = part2_recurse(&mut iter);
    println!("Part 2: {}", result);
}

fn part2_recurse(iter: &mut Iter<usize>) -> usize {
    let n_children = *iter.next().unwrap();
    let n_metas = *iter.next().unwrap();
    let mut values: Vec<usize> = vec![0; n_children];
    for idx in 0..n_children {
        values[idx] = part2_recurse(iter);
    }
    let mut value: usize = 0;
    for _ in 0..n_metas {
        let idx = *iter.next().unwrap();
        if n_children == 0 {
            value += idx;
        }
        else if idx > 0 && idx <= n_children {
            value += values[idx - 1];
        }
    }
    value
}

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

fn part1(input: &[usize]) -> usize {
    let mut iter = input.iter();
    part1_recurse(&mut iter)
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

fn part2(input: &[usize]) -> usize {
    let mut iter = input.iter();
    part2_recurse(&mut iter)
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

fn main() {
    let input = read_input::<Input>();
    println!("Part 1: {}", part1(&(input[0].list)));
    println!("Part 2: {}", part2(&(input[0].list)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day08_test() {
        let input: Vec<Input> = test_input("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2".into());
        assert_eq!(part1(&input[0].list), 138);
        assert_eq!(part2(&input[0].list), 66);
    }
}

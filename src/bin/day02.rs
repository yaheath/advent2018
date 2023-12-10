use std::vec::Vec;
use counter::Counter;
use itertools::Itertools;
use advent_lib::read::read_input;

fn part1(input: &Vec<String>) -> i32 {
    let (twos, threes) = input.iter()
        .map(|line| line.chars().collect::<Counter<_>>())
        .map(|counter| counter.values().fold((0,0), |(two,thr), v| match v {
            2 => ((two+1).min(1), thr),
            3 => (two, (thr+1).min(1)),
            _ => (two, thr),
        }))
        .fold((0,0), |a,b| (a.0+b.0, a.1+b.1));
    twos * threes
}

fn part2(input: &Vec<String>) -> String {
    input.iter()
        .cartesian_product(input.iter())
        .filter_map(|(a, b)| check_string_diff(a, b))
        .next()
        .unwrap()
}

fn check_string_diff(s1: &String, s2: &String) -> Option<String> {
    let common: String = s1.chars()
        .zip(s2.chars())
        .filter_map(|(c1, c2)| if c1 == c2 { Some(c1) } else { None })
        .collect();
    if common.len() == s1.len() - 1 {
        Some(common)
    }
    else {
        None
    }
}

fn main() {
    let input: Vec<String> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day02_test() {
        let input: Vec<String> = test_input(include_str!("day02.testinput"));
        assert_eq!(part1(&input), 12);
        let input: Vec<String> = test_input(include_str!("day02.testinput2"));
        assert_eq!(part2(&input), String::from("fgij"));
    }
}

use std::collections::HashSet;
use std::vec::Vec;
use ya_advent_lib::read::read_input;

fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

fn part2(input: &[i32]) -> i32 {
    let mut freq: i32 = 0;
    let mut set: HashSet<i32> = HashSet::new();
    set.insert(freq);
    for n in input.iter().cycle() {
        freq += n;
        if set.contains(&freq) {
            return freq;
        }
        set.insert(freq);
    }
    panic!();
}

fn main() {
    let input: Vec<i32> = read_input();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day01_test() {
        let input: Vec<i32> = test_input("+1\n-2\n+3\n+1\n");
        assert_eq!(part1(&input), 3);
        let input: Vec<i32> = test_input("+1\n+1\n+1\n");
        assert_eq!(part1(&input), 3);
        let input: Vec<i32> = test_input("+1\n+1\n-2\n");
        assert_eq!(part1(&input), 0);
        let input: Vec<i32> = test_input("-1\n-2\n-3\n");
        assert_eq!(part1(&input), -6);

        let input: Vec<i32> = test_input("+1\n-2\n+3\n+1\n");
        assert_eq!(part2(&input), 2);
        let input: Vec<i32> = test_input("+1\n-1\n");
        assert_eq!(part2(&input), 0);
        let input: Vec<i32> = test_input("+3\n+3\n+4\n-2\n-4\n");
        assert_eq!(part2(&input), 10);
        let input: Vec<i32> = test_input("-6\n+3\n+8\n+5\n-6\n");
        assert_eq!(part2(&input), 5);
        let input: Vec<i32> = test_input("+7\n+7\n-2\n-7\n-4\n");
        assert_eq!(part2(&input), 14);

    }
}

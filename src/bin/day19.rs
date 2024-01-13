use advent_lib::read::read_input;
extern crate advent2018;
use advent2018::vm::{VM, ProgramItem, RunResult};

fn main() {
    let prog: Vec<ProgramItem> = read_input();

    println!("Part 1: {}", part1(&prog));
    println!("Part 2: {}", part2(&prog));
}

fn part1(prog: &[ProgramItem]) -> usize {
    let mut vm = VM::new();
    vm.load(prog);
    match vm.run() {
        RunResult::Err(e) => panic!("Error while running program: {}", e),
        RunResult::Halt => {return vm.r[0];},
        _ => (),
    }
    panic!();
}

fn part2(prog: &[ProgramItem]) -> usize {
    let mut vm = VM::new();
    vm.load(prog);
    vm.r[0] = 1;
    // The program calculates the sum of all divisors of a large number.
    // It's too slow to complete in a reasonable amount of time, so we'll figure
    // out what number it's using and then do it directly instead of by continuing
    // to run the program.
    // I can't take credit for figuring that out; I didn't have the patience to
    // analyze what the program was doing and found it on reddit.
    let mut last_ip = 0usize;
    loop {
        match vm.step() {
            RunResult::Err(e) => panic!("Error while running program: {}", e),
            RunResult::Halt => println!("Part 2: {}", vm.r[0]),
            _ => ()
        };
        // when the IP goes backward the first time, the program is done initializing
        // the number it wants to factor, which will be in register 4
        if vm.r[vm.ip] < last_ip { break; }
        last_ip = vm.r[vm.ip];
    }
    let bignum = vm.r[4];
    (1..=bignum).filter(|i| bignum % i == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day19_test() {
        let input: Vec<ProgramItem> = test_input(include_str!("day19.testinput"));
        assert_eq!(part1(&input), 7);
    }
}

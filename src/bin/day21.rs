use std::collections::HashSet;
use std::vec::Vec;
use advent_lib::read::read_input;
extern crate advent2018;
use advent2018::vm::{VM, ProgramItem, RunResult};

fn main() {
    let prog: Vec<ProgramItem> = read_input();

    let mut values: HashSet<usize> = HashSet::new();
    let mut vm = VM::new();
    let mut last = 0usize;
    vm.load(&prog);
    vm.set_breakpoint(0);
    loop {
        match vm.run() {
            RunResult::Break(inst) => {
                //println!("hit breakpoint: {inst:?} {:?}", vm.r);
                assert_eq!(inst.opcode, "eqrr");
                let target = if inst.a == 0 { vm.r[inst.b] } else { vm.r[inst.a] };
                if values.len() == 0 {
                    println!("Part 1: {target}");
                }
                if values.contains(&target) {
                    println!("Part 2: {last}");
                    return;
                }
                values.insert(target);
                last = target;
            },
            _ => panic!(),
        };
    }
}

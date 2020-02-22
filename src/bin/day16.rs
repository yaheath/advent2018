#[macro_use] extern crate lazy_static;
use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use regex::Regex;
extern crate advent;
use advent::read::input_lines;

struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}
impl Instruction {
    fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE_INST: Regex = Regex::new(
                r"(\d+) (\d+) (\d+) (\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_INST.captures(s) {
            let o = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            let a = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let b = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let c = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            Some(Self {
                opcode: o,
                a: a,
                b: b,
                c: c,
            })
        }
        else {
            None
        }
    }
}

struct VM {
    r: [usize; 4],
}
impl VM {
    fn new() -> Self {
        VM { r: [0; 4] }
    }
    fn with_reg(regs: &[usize; 4]) -> Self {
        VM { r: *regs }
    }
    fn exec(&mut self, op: &Op, inst: &Instruction) -> Result<(), &'static str> {
        if inst.c > 3 {
            Err("C out of range")
        } else if !op.a_immed && inst.a > 3 {
            Err("A out of range")
        } else if !op.b_immed && inst.b > 3 {
            Err("B out of range")
        } else {
            let a = if op.a_immed { inst.a } else { self.r[inst.a] };
            let b = if op.b_immed { inst.b } else { self.r[inst.b] };
            self.r[inst.c] = (op.op)(a, b);
            Ok(())
        }
    }
}

struct Op {
    a_immed: bool,
    b_immed: bool,
    op: &'static dyn Fn(usize, usize) -> usize,
}

fn make_operations() -> HashMap<&'static str, Op> {
    let mut m = HashMap::new();
    m.insert("addr", Op{a_immed:false, b_immed:false, op:&|a, b| a + b});
    m.insert("addi", Op{a_immed:false, b_immed:true,  op:&|a, b| a + b});
    m.insert("mulr", Op{a_immed:false, b_immed:false, op:&|a, b| a * b});
    m.insert("muli", Op{a_immed:false, b_immed:true,  op:&|a, b| a * b});
    m.insert("banr", Op{a_immed:false, b_immed:false, op:&|a, b| a & b});
    m.insert("bani", Op{a_immed:false, b_immed:true,  op:&|a, b| a & b});
    m.insert("borr", Op{a_immed:false, b_immed:false, op:&|a, b| a | b});
    m.insert("bori", Op{a_immed:false, b_immed:true,  op:&|a, b| a | b});
    m.insert("setr", Op{a_immed:false, b_immed:true,  op:&|a, _| a});
    m.insert("seti", Op{a_immed:true,  b_immed:true,  op:&|a, _| a});
    m.insert("gtir", Op{a_immed:true,  b_immed:false, op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert("gtri", Op{a_immed:false, b_immed:true,  op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert("gtrr", Op{a_immed:false, b_immed:false, op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert("eqir", Op{a_immed:true,  b_immed:false, op:&|a, b| if a == b { 1 } else { 0 }});
    m.insert("eqri", Op{a_immed:false, b_immed:true,  op:&|a, b| if a == b { 1 } else { 0 }});
    m.insert("eqrr", Op{a_immed:false, b_immed:false, op:&|a, b| if a == b { 1 } else { 0 }});
    m
}

fn registers_from_str(s: &str) -> Option<[usize; 4]> {
    lazy_static! {
        static ref RE_REGS: Regex = Regex::new(
            r"\[(\d+), (\d+), (\d+), (\d+)\]",
        ).unwrap();
    }
    if let Some(caps) = RE_REGS.captures(s) {
        let a = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let b = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let c = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        let d = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
        Some([a, b, c, d])
    }
    else {
        None
    }
}

struct Sample {
    inst: Instruction,
    before: [usize; 4],
    after: [usize; 4],
}

fn main() {
    let operations = make_operations();
    let mut lineiter = input_lines();
    let mut samples: Vec<Sample> = Vec::new();

    loop {
        let line = lineiter.next().unwrap().unwrap();
        if line.len() == 0 { break; }
        let before = registers_from_str(&line).unwrap();
        let line = lineiter.next().unwrap().unwrap();
        let inst = Instruction::from_str(&line).unwrap();
        let line = lineiter.next().unwrap().unwrap();
        let after = registers_from_str(&line).unwrap();
        samples.push(Sample { before: before, inst: inst, after: after });
        lineiter.next();
    }
    let program:Vec<Instruction> = lineiter
        .map(|l| Instruction::from_str(&l.unwrap()))
        .filter(|i| i.is_some())
        .map(|i| i.unwrap())
        .collect();

    let mut oper_table: Vec<HashSet<&'static str>> = Vec::with_capacity(16);
    for _ in 0..16 { oper_table.push(HashSet::new()); }

    let count = samples.iter().fold(0, |count, sample| {
        let syms = test_sample(sample, &operations);
        for s in syms.iter() {
            oper_table[sample.inst.opcode].insert(s);
        }
        if syms.len() >= 3 {
            count + 1
        } else {
            count
        }
    });
    println!("Part 1: {}", count);

    /*
    for (idx, set) in oper_table.iter().enumerate() {
        let ops: Vec<String> = set.iter().map(|s| (*s).to_string()).collect();
        println!("{}: {}", idx, ops.join(" "));
    }*/

    let mut final_oper_table: Vec<&'static str> = vec![&""; 16];
    loop {
        let mut item: Option<&'static str> = None;
        {
            if let Some((i, h)) = oper_table.iter().enumerate().find(|(_, h)| h.len() == 1) {
                item = Some(*(h.iter().nth(0).unwrap()));
                final_oper_table[i] = item.unwrap();
            }
        }
        if let Some(item) = item {
            for h in oper_table.iter_mut() {
                h.remove(item);
            }
        }
        else if oper_table.iter().all(|h| h.len() == 0) {
            break;
        }
        else {
            panic!("no opers remain that are narrowed down to one");
        }
    }

    let mut vm = VM::new();
    for inst in program.iter() {
        let op = final_oper_table[inst.opcode];
        vm.exec(operations.get(op).unwrap(), inst).unwrap();
    }
    println!("Part 2: {}", vm.r[0]);
}

fn test_sample(sample: &Sample, operations: &HashMap<&'static str, Op>) -> Vec<&'static str> {
    let mut syms: Vec<&'static str> = Vec::new();
    for (sym, op) in operations {
        let mut vm = VM::with_reg(&sample.before);
        if vm.exec(op, &sample.inst).is_ok() {
            if vm.r == sample.after {
                syms.push(*sym);
            }
        }
    }
    syms
}

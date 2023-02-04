#[macro_use] extern crate lazy_static;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use advent_lib::read::read_input;

const NREGS: usize = 6;

#[derive(Clone, Copy)]
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
                r"(\w+) (\d+) (\d+) (\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_INST.captures(s) {
            let o = caps.get(1).unwrap().as_str();
            let a = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
            let b = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
            let c = caps.get(4).unwrap().as_str().parse::<usize>().unwrap();
            Some(Self {
                opcode: Self::encode_opcode(o),
                a: a,
                b: b,
                c: c,
            })
        }
        else {
            None
        }
    }
    fn encode_opcode(s: &str) -> usize {
        s.chars().fold(0, |v, c| (v << 8) | (c as usize))
    }
}

enum Meta {
    MapIp(usize),
}
impl Meta {
    fn from_str(s: &str) -> Option<Self> {
        lazy_static! {
            static ref RE_IP: Regex = Regex::new(
                r"^#ip (\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_IP.captures(s) {
            let arg = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            Some(Meta::MapIp(arg))
        }
        else {
            None
        }
    }
}

enum ProgramItem {
    Instr(Instruction),
    Meta(Meta),
}
impl FromStr for ProgramItem {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(inst) = Instruction::from_str(s) {
            Ok(ProgramItem::Instr(inst))
        }
        else if let Some(meta) = Meta::from_str(s) {
            Ok(ProgramItem::Meta(meta))
        }
        else {
            Err("invalid input")
        }
    }
}

enum RunResult {
    Ok,
    Halt,
    InfiniteLoop,
    Err(&'static str),
}

struct VM<'a> {
    r: [usize; NREGS],
    ip: usize,
    prog: Vec<Instruction>,
    imap: &'a HashMap<usize, Op>,
    counter: usize,
}
impl<'a> VM<'a> {
    fn new(instr_map: &'a HashMap<usize, Op>) -> Self {
        VM {
            r: [0; NREGS],
            ip: 0,
            prog: Vec::new(),
            imap: instr_map,
            counter: 0,
        }
    }
    fn load(&mut self, program: &Vec<ProgramItem>) {
        self.prog.clear();
        for pi in program {
            match pi {
                ProgramItem::Instr(inst) => self.prog.push(*inst),
                ProgramItem::Meta(meta) =>
                    match meta {
                        Meta::MapIp(val) => self.ip = *val,
                    }
            }
        }
    }
    fn exec(&mut self, inst: &Instruction) -> Result<(), &'static str> {
        if !self.imap.contains_key(&inst.opcode) {
            return Err("Illegal instruction");
        }
        let op = &self.imap[&inst.opcode];
        if inst.c >= NREGS {
            Err("C out of range")
        } else if !op.a_immed && inst.a >= NREGS {
            Err("A out of range")
        } else if !op.b_immed && inst.b >= NREGS {
            Err("B out of range")
        } else {
            let a = if op.a_immed { inst.a } else { self.r[inst.a] };
            let b = if op.b_immed { inst.b } else { self.r[inst.b] };
            self.r[inst.c] = (op.op)(a, b);
            self.counter += 1;
            Ok(())
        }
    }
    fn step(&mut self) -> RunResult {
        if self.r[self.ip] >= self.prog.len() {
            return RunResult::Halt;
        }
        if self.r[self.ip] == 6 {
            print!("");
        }
        let inst = self.prog[self.r[self.ip]];
        if inst.opcode == Instruction::encode_opcode("seti") &&
            inst.a == 0 && inst.c == 2 {
                return RunResult::InfiniteLoop;
        }
        match self.exec(&inst) {
            Ok(()) => {
                self.r[self.ip] += 1;
                RunResult::Ok
            },
            Err(e) => RunResult::Err(e),
        }
    }
    fn run(&mut self) -> RunResult {
        loop {
            let res = self.step();
            match res {
                RunResult::Ok => (),
                _ => return res,
            };
            /*
            if verbose {
                print!("\x1b7{} {} {} {} {} {}\x1b[K\x1b8",
                       self.r[0], self.r[1], self.r[2], self.r[3], self.r[4], self.r[5]);
            }
            */
        }
    }
    fn reset(&mut self, regzero: usize) {
        self.r = [0; NREGS];
        self.r[0] = regzero;
        self.counter = 0;
    }
}

struct Op {
    a_immed: bool,
    b_immed: bool,
    op: &'static dyn Fn(usize, usize) -> usize,
}

fn make_operations() -> HashMap<usize, Op> {
    let mut m = HashMap::new();
    let e = |s| Instruction::encode_opcode(s);
    m.insert(e("addr"), Op{a_immed:false, b_immed:false, op:&|a, b| a + b});
    m.insert(e("addi"), Op{a_immed:false, b_immed:true,  op:&|a, b| a + b});
    m.insert(e("mulr"), Op{a_immed:false, b_immed:false, op:&|a, b| a * b});
    m.insert(e("muli"), Op{a_immed:false, b_immed:true,  op:&|a, b| a * b});
    m.insert(e("banr"), Op{a_immed:false, b_immed:false, op:&|a, b| a & b});
    m.insert(e("bani"), Op{a_immed:false, b_immed:true,  op:&|a, b| a & b});
    m.insert(e("borr"), Op{a_immed:false, b_immed:false, op:&|a, b| a | b});
    m.insert(e("bori"), Op{a_immed:false, b_immed:true,  op:&|a, b| a | b});
    m.insert(e("setr"), Op{a_immed:false, b_immed:true,  op:&|a, _| a});
    m.insert(e("seti"), Op{a_immed:true,  b_immed:true,  op:&|a, _| a});
    m.insert(e("gtir"), Op{a_immed:true,  b_immed:false, op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert(e("gtri"), Op{a_immed:false, b_immed:true,  op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert(e("gtrr"), Op{a_immed:false, b_immed:false, op:&|a, b| if a > b  { 1 } else { 0 }});
    m.insert(e("eqir"), Op{a_immed:true,  b_immed:false, op:&|a, b| if a == b { 1 } else { 0 }});
    m.insert(e("eqri"), Op{a_immed:false, b_immed:true,  op:&|a, b| if a == b { 1 } else { 0 }});
    m.insert(e("eqrr"), Op{a_immed:false, b_immed:false, op:&|a, b| if a == b { 1 } else { 0 }});
    m
}

fn main() {
    let operations = make_operations();
    let prog: Vec<ProgramItem> = read_input();

    part1(&prog, &operations);
    //part2(&prog, &operations);
}

fn part1(prog: &Vec<ProgramItem>, operations: &HashMap<usize, Op>) {
    let mut vm = VM::new(operations);
    vm.load(prog);
    for n in 0..100 {
        vm.reset(n);
        match vm.run() {
            RunResult::Err(e) => panic!("Error while running program: {}", e),
            RunResult::Halt => println!("n={} count={}", n, vm.r[0]),
            RunResult::InfiniteLoop => println!("n={} infiniteloop", n),
            _ => ()
        }
    };
}

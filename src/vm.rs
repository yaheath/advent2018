use std::collections::HashMap;
use std::str::FromStr;
use std::vec::Vec;
use regex::Regex;
use lazy_static::lazy_static;

const NREGS: usize = 6;

#[derive(Clone, Copy)]
pub struct Instruction {
    opcode: &'static str,
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
                opcode: OPERATIONS.get_key_value(o).unwrap().0,
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

pub enum Meta {
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

pub enum ProgramItem {
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

pub enum RunResult {
    Ok,
    Halt,
    Err(&'static str),
}

pub struct VM {
    r: [usize; NREGS],
    ip: usize,
    prog: Vec<Instruction>,
}
impl VM {
    pub fn new() -> Self {
        VM {
            r: [0; NREGS],
            ip: 0,
            prog: Vec::new(),
        }
    }
    pub fn load(&mut self, program: &Vec<ProgramItem>) {
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
    pub fn exec(&mut self, inst: &Instruction) -> Result<(), &'static str> {
        if !OPERATIONS.contains_key(&inst.opcode) {
            return Err("Illegal instruction");
        }
        let op = &OPERATIONS[&inst.opcode];
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
            Ok(())
        }
    }
    pub fn step(&mut self) -> RunResult {
        if self.r[self.ip] >= self.prog.len() {
            return RunResult::Halt;
        }
        if self.r[self.ip] == 6 {
            print!("");
        }
        let inst = self.prog[self.r[self.ip]];
        match self.exec(&inst) {
            Ok(()) => {
                self.r[self.ip] += 1;
                RunResult::Ok
            },
            Err(e) => RunResult::Err(e),
        }
    }
    pub fn run(&mut self) -> RunResult {
        loop {
            let res = self.step();
            match res {
                RunResult::Err(e) => return RunResult::Err(e),
                RunResult::Halt => return RunResult::Halt,
                RunResult::Ok => (),
            };
            /*
            if verbose {
                print!("\x1b7{} {} {} {} {} {}\x1b[K\x1b8",
                       self.r[0], self.r[1], self.r[2], self.r[3], self.r[4], self.r[5]);
            }
            */
        }
    }
}

struct Op {
    a_immed: bool,
    b_immed: bool,
    op: fn(usize, usize) -> usize,
}

lazy_static! {
    static ref OPERATIONS: HashMap<&'static str, Op> =
        HashMap::from_iter([
            ("addr", Op{a_immed:false, b_immed:false, op:|a, b| a + b}),
            ("addi", Op{a_immed:false, b_immed:true,  op:|a, b| a + b}),
            ("mulr", Op{a_immed:false, b_immed:false, op:|a, b| a * b}),
            ("muli", Op{a_immed:false, b_immed:true,  op:|a, b| a * b}),
            ("banr", Op{a_immed:false, b_immed:false, op:|a, b| a & b}),
            ("bani", Op{a_immed:false, b_immed:true,  op:|a, b| a & b}),
            ("borr", Op{a_immed:false, b_immed:false, op:|a, b| a | b}),
            ("bori", Op{a_immed:false, b_immed:true,  op:|a, b| a | b}),
            ("setr", Op{a_immed:false, b_immed:true,  op:|a, _| a}),
            ("seti", Op{a_immed:true,  b_immed:true,  op:|a, _| a}),
            ("gtir", Op{a_immed:true,  b_immed:false, op:|a, b| if a > b  { 1 } else { 0 }}),
            ("gtri", Op{a_immed:false, b_immed:true,  op:|a, b| if a > b  { 1 } else { 0 }}),
            ("gtrr", Op{a_immed:false, b_immed:false, op:|a, b| if a > b  { 1 } else { 0 }}),
            ("eqir", Op{a_immed:true,  b_immed:false, op:|a, b| if a == b { 1 } else { 0 }}),
            ("eqri", Op{a_immed:false, b_immed:true,  op:|a, b| if a == b { 1 } else { 0 }}),
            ("eqrr", Op{a_immed:false, b_immed:false, op:|a, b| if a == b { 1 } else { 0 }}),
        ]);
}

use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin, StdinLock};
use std::iter::Iterator;
use std::str::FromStr;
use std::vec::Vec;

lazy_static! {
    static ref STDIN: Stdin = io::stdin();
}

fn stdinlock() -> StdinLock<'static> {
    STDIN.lock()
}

enum LineIters {
    File(Lines<BufReader<File>>),
    Stdin(Lines<StdinLock<'static>>),
}

pub struct LineIter {
    inner: LineIters,
}
impl Iterator for LineIter {
    type Item = io::Result<String>;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            LineIters::File(i) => i.next(),
            LineIters::Stdin(i) => i.next(),
        }
    }
}

pub fn input_lines() -> LineIter {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        LineIter {
            inner: LineIters::File(
                BufReader::new(file).lines()
            ),
        }
    } else {
        let lock = stdinlock();
        LineIter {
            inner: LineIters::Stdin(lock.lines()),
        }
    }
}

pub fn read_input<T: FromStr>() -> Vec<T> {
    let mut data: Vec<T> = Vec::new();
    for line in input_lines() {
        match line {
            Ok(line) => {
                match line.trim_end().parse::<T>() {
                    Ok(val) => data.push(val),
                    Err(_) => eprintln!("Invalid line: {}", line.trim()),
                }
            },
            Err(e) => {
                eprintln!("Error reading stdin: {}", e);
                break;
            },
        };
    };
    return data;
}

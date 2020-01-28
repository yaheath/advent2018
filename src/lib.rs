use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::str::FromStr;
use std::vec::Vec;

pub fn read_input<T: FromStr>() -> Vec<T> {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let file = File::open(&args[1]).unwrap();
        return read_from(BufReader::new(file));
    }
    else {
        return read_from(io::stdin().lock());
    }
}

fn read_from<T: FromStr>(reader: impl BufRead) -> Vec<T> {
    let mut data: Vec<T> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                match line.trim().parse::<T>() {
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

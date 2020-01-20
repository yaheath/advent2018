use std::io::{self, BufRead};
use std::str::FromStr;
use std::vec::Vec;

pub fn read_input<T: FromStr>() -> Vec<T> {
    let mut data: Vec<T> = Vec::new();
    for line in io::stdin().lock().lines() {
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

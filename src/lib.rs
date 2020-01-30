use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::slice::Iter;
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

pub struct Grid {
    min_x: i32,
    min_y: i32,
    x_size: usize,
    y_size: usize,
    data: Vec<i32>,
}

impl Grid {
    pub fn new(min_x:i32, min_y:i32, max_x:i32, max_y:i32) -> Self {
        let x_size = (max_x - min_x + 1) as usize;
        let y_size = (max_y - min_y + 1) as usize;
        let mut g = Self {
            min_x: min_x,
            min_y: min_y,
            x_size: x_size,
            y_size: y_size,
            data: Vec::with_capacity(x_size * y_size),
        };
        for _ in 0..x_size * y_size {
            g.data.push(-1);
        }
        g
    }

    pub fn get(&self, x:i32, y:i32) -> i32 {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn set(&mut self, x:i32, y:i32, val:i32) {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
    }

    pub fn iter(&self) -> Iter<i32> {
        self.data.iter()
    }
}

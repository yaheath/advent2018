use std::cmp::max;
use std::io::Write;
use std::ops::Range;
use std::slice::Iter;
use std::vec::Vec;

pub struct Grid<T: Copy> {
    min_x: i32,
    min_y: i32,
    x_size: usize,
    y_size: usize,
    data: Vec<T>,
}

impl<T: Copy> Grid<T> {
    pub fn new(min_x:i32, min_y:i32, max_x:i32, max_y:i32, initial_val: T) -> Self {
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
            g.data.push(initial_val);
        }
        g
    }

    pub fn from_input<F>(input: &Vec<String>, default_val: T, padding: i32, mapfunc: F) -> Self
            where F: Fn(char) -> T {
        let width = input.iter().map(|s| s.len()).fold(0, |maxw, w| max(w, maxw)) as i32;
        let height = input.len() as i32;
        let mut y = 0i32;
        let mut inst = Self::new(-padding, -padding, width-1+padding, height-1+padding, default_val);
        for line in input.iter() {
            for (ux, c) in line.chars().enumerate() {
                let x = ux as i32;
                inst.set(x, y, mapfunc(c));
            }
            y += 1;
        }
        inst
    }

    pub fn clone(&self) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: self.data.clone(),
        }
    }

    pub fn clone_without_data(&self, initial_val: T) -> Self {
        Grid {
            min_x: self.min_x,
            min_y: self.min_y,
            x_size: self.x_size,
            y_size: self.y_size,
            data: vec![initial_val; self.data.len()],
        }
    }

    pub fn get(&self, x:i32, y:i32) -> T {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx]
    }

    pub fn set(&mut self, x:i32, y:i32, val:T) {
        assert!(x >= self.min_x && x <= self.min_x + self.x_size as i32);
        assert!(y >= self.min_y && y <= self.min_y + self.y_size as i32);
        let ux:usize = (x - self.min_x) as usize;
        let uy:usize = (y - self.min_y) as usize;
        let idx = uy * self.x_size + ux;
        self.data[idx] = val;
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn x_bounds(&self) -> Range<i32> {
        self.min_x .. self.min_x + self.x_size as i32
    }

    pub fn y_bounds(&self) -> Range<i32> {
        self.min_y .. self.min_y + self.y_size as i32
    }

    pub fn dump_to_file<F>(&self, file: &mut dyn Write, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i32 {
            for x in self.min_x .. self.min_x + self.x_size as i32 {
                write!(file, "{}", formatter(self.get(x, y))).unwrap();
            }
            writeln!(file, "").unwrap();
        }
    }

    pub fn print<F>(&self, formatter: F)
            where F: Fn(T) -> char {
        for y in self.min_y .. self.min_y + self.y_size as i32 {
            for x in self.min_x .. self.min_x + self.x_size as i32 {
                print!("{}", formatter(self.get(x, y)));
            }
            println!("");
        }
    }
}

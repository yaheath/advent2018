use std::cmp::{max, PartialEq};
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::ops::{Bound, Index, IndexMut, RangeBounds};
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

pub struct NumberLine<T: Copy + PartialEq> {
    min_idx: i64,
    max_idx: i64,
    data: Vec<T>,
    default_val: T,
}
impl<T: Copy + PartialEq> NumberLine<T> {
    pub fn new(min_idx: i64, max_idx: i64, default_val: T) -> Self {
        assert!(max_idx >= min_idx);
        Self {
            min_idx: min_idx,
            max_idx: max_idx,
            data: vec![default_val; (max_idx - min_idx + 1) as usize],
            default_val: default_val,
        }
    }
    pub fn from_initial(initial: &Vec<T>, default_val: T) -> Self {
        let mut data: Vec<T> = vec![default_val; initial.len()];
        data.copy_from_slice(&initial[..]);
        Self {
            min_idx: 0,
            max_idx: data.len() as i64 - 1,
            data: data,
            default_val: default_val,
        }
    }
    pub fn len(&self) -> usize { self.data.len() }
    pub fn start_index(&self) -> i64 { self.min_idx }
    pub fn end_index(&self) -> i64 { self.max_idx + 1 }
    pub fn clone(&self) -> Self {
        let mut data: Vec<T> = vec![self.default_val; self.data.len()];
        data.copy_from_slice(&self.data[..]);
        Self {
            min_idx: self.min_idx,
            max_idx: self.max_idx,
            data: data,
            default_val: self.default_val,
        }
    }
    pub fn iter(&self) -> Iter<T> { self.data.iter() }
    pub fn enumerate(&self) -> NumberLineEnumerator<T> {
        NumberLineEnumerator::new(&self)
    }
}
impl<T: Copy + PartialEq> Index<i64> for NumberLine<T> {
    type Output = T;
    fn index(&self, idx: i64) -> &Self::Output {
        if idx < self.min_idx || idx > self.max_idx {
            &self.default_val
        }
        else {
            &self.data[(idx - self.min_idx) as usize]
        }
    }
}
impl<T: Copy + PartialEq> IndexMut<i64> for NumberLine<T> {
    fn index_mut(&mut self, idx: i64) -> &mut Self::Output {
        let datalen = self.data.len() as i64;
        if idx < self.min_idx {
            let ext = max(self.min_idx - idx, 32);
            let newlen = datalen + ext;
            self.data.resize(newlen as usize, self.default_val);
            self.data.copy_within(0 .. (newlen - ext) as usize, ext as usize);
            for i in 0 .. ext as usize {
                self.data[i] = self.default_val
            }
            self.min_idx -= ext;
        }
        else if idx - self.min_idx >= datalen {
            let ext = max(idx - self.min_idx - datalen + 1, 32);
            let newlen = datalen + ext;
            self.data.resize(newlen as usize, self.default_val);
            self.max_idx += ext;
        }
        &mut self.data[(idx - self.min_idx) as usize]
    }
}
impl<T: Copy + PartialEq> RangeBounds<i64> for NumberLine<T> {
    fn start_bound(&self) -> Bound<&i64> { Bound::Included(&self.min_idx) }
    fn end_bound(&self) -> Bound<&i64> { Bound::Included(&self.max_idx) }
}

pub struct NumberLineEnumerator<'a, T: Copy + PartialEq> {
    obj: &'a NumberLine<T>,
    idx: i64,
    max_idx: i64,
}
impl<'a, T: Copy + PartialEq> NumberLineEnumerator<'a, T> {
    pub fn new(obj: &'a NumberLine<T>) -> Self {
        let mut min_idx = obj.min_idx;
        let mut max_idx = obj.min_idx - 1;
        if let Some(end) = obj.iter().rev().enumerate()
                              .find(|x| *(x.1) != obj.default_val) {
            max_idx = obj.min_idx + (obj.data.len() - end.0 - 1) as i64;
            while obj[min_idx] == obj.default_val {
                min_idx += 1;
            }
        }
        Self {
            obj: obj,
            idx: min_idx,
            max_idx: max_idx,
        }
    }
}
impl<T: Copy + PartialEq> Iterator for NumberLineEnumerator<'_, T> {
    type Item = (i64, T);
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx > self.max_idx {
            None
        }
        else {
            let r = (self.idx, self.obj[self.idx]);
            self.idx += 1;
            Some(r)
        }
    }
}

#[cfg(test)]
mod test {
    use super::NumberLine;

    #[test]
    fn numberline() {
        let mut foo: NumberLine<bool> = NumberLine::new(1, 10, false);
        assert_eq!(foo[0], false);
        assert_eq!(foo[1], false);
        assert_eq!(foo[10], false);
        assert_eq!(foo[333], false);
        assert_eq!(foo[-333], false);
        assert_eq!(foo.len(), 10);

        foo[2] = true;
        assert_eq!(foo[2], true);
        assert_eq!(foo.len(), 10);
        foo[4] = true;

        let mut e = foo.enumerate();
        assert_eq!(e.next(), Some((2, true)));
        assert_eq!(e.next(), Some((3, false)));
        assert_eq!(e.next(), Some((4, true)));
        assert_eq!(e.next(), None);

        foo[-2] = true;
        assert_eq!(foo[-2], true);
        assert_eq!(foo[-1], false);
        assert_eq!(foo[0], false);
        assert_eq!(foo[1], false);
        assert_eq!(foo[2], true);
        assert_eq!(foo[3], false);
        assert_eq!(foo.len(), 42);

        foo[20] = true;
        assert_eq!(foo[2], true);
        assert_eq!(foo[19], false);
        assert_eq!(foo[20], true);
        assert_eq!(foo[21], false);
        assert_eq!(foo.len(), 74);
    }
}

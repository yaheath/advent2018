#[macro_use] extern crate lazy_static;
use std::cell::RefCell;
use std::cmp::{max, min};
use std::io::BufWriter;
use std::ops::{Range, RangeInclusive};
use std::str::FromStr;
use regex::Regex;
extern crate advent;
use advent::read::read_input;
use advent::grid::Grid;

enum InputItem {
    Row(RangeInclusive<i32>, i32),
    Col(i32, RangeInclusive<i32>),
}

impl FromStr for InputItem {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_ROW: Regex = Regex::new(
                r"y=(\d+), x=(\d+)\.\.(\d+)",
            ).unwrap();
        }
        lazy_static! {
            static ref RE_COL: Regex = Regex::new(
                r"x=(\d+), y=(\d+)\.\.(\d+)",
            ).unwrap();
        }
        if let Some(caps) = RE_ROW.captures(s) {
            let y = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let x1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let x2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            return Ok(InputItem::Row(x1..=x2, y));
        }
        if let Some(caps) = RE_COL.captures(s) {
            let x = caps.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let y1 = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
            let y2 = caps.get(3).unwrap().as_str().parse::<i32>().unwrap();
            return Ok(InputItem::Col(x, y1..=y2));
        }
        Err("invalid input line".to_string())
    }
}

#[derive(Clone, Copy)]
enum WCell {
    Sand,
    Clay,
    WetSand,
    Water,
}

struct WrappedGrid<T: Copy> {
    grid: RefCell<Grid<T>>,
}
impl<T: Copy> WrappedGrid<T> {
    fn new(grid: Grid<T>) -> Self {
        WrappedGrid{ grid: RefCell::new(grid) }
    }
    fn get(&self, x:i32, y:i32) -> T {
        self.grid.borrow().get(x, y)
    }
    fn set(&self, x:i32, y:i32, val:T) {
        self.grid.borrow_mut().set(x, y, val);
    }
    /*
    fn x_bounds(&self) -> Range<i32> {
        self.grid.borrow().x_bounds()
    }*/
    fn y_bounds(&self) -> Range<i32> {
        self.grid.borrow().y_bounds()
    }
}

fn main() {
    let input = read_input::<InputItem>();
    let (minx, maxx) = input.iter().fold((std::i32::MAX, 0),
        |(minx, maxx), item| match item {
            InputItem::Row(x, _) => (min(minx, *x.start()), max(maxx, *x.end())),
            InputItem::Col(x, _) => (min(minx, *x), max(maxx, *x)),
        }
    );
    let (miny, maxy) = input.iter().fold((std::i32::MAX, 0),
        |(miny, maxy), item| match item {
            InputItem::Row(_, y) => (min(miny, *y), max(maxy, *y)),
            InputItem::Col(_, y) => (min(miny, *y.start()), max(maxy, *y.end())),
        }
    );
    let mut grid = Grid::new(minx-1, miny, maxx+1, maxy, WCell::Sand);
    for item in input.iter() {
        match item {
            InputItem::Row(xr, y) =>
                for x in xr.clone() {
                    grid.set(x, *y, WCell::Clay);
                },
            InputItem::Col(x, yr) =>
                for y in yr.clone() {
                    grid.set(*x, y, WCell::Clay);
                },
        }
    }

    //println!("{},{} {},{}", minx-1, miny, maxx+1, maxy);
    grid.set(500, miny, WCell::WetSand);

    let grid = WrappedGrid::new(grid);
    go_vertical(500, miny, &grid);
    dump_grid(&grid.grid.borrow());

    let (n_w, n_s) = grid.grid.borrow().iter().fold((0, 0), |(n_w, n_s), cell| match cell {
        WCell::Water => (n_w + 1, n_s),
        WCell::WetSand => (n_w, n_s + 1),
        _ => (n_w, n_s),
    });
    println!("Part 1: {}", n_w + n_s);
    println!("Part 2: {}", n_w);
}

fn go_vertical(start_x:i32, start_y:i32, grid: &WrappedGrid<WCell>) -> bool {
    //dump_grid(&grid.grid.borrow());
    let x = start_x;
    let mut y = start_y;
    loop {
        grid.set(x, y, WCell::WetSand);
        if y + 1 == grid.y_bounds().end {
            return true;
        }
        match grid.get(x, y + 1) {
            WCell::Sand => { y += 1; },
            WCell::WetSand => { return true; }
            WCell::Clay | WCell::Water   => { break; },
        }
    }
    loop {
        if go_horiz(x, y, &grid) { return true; }
        y -= 1;
        if y <= start_y { return false; }
    }
}

fn go_horiz(start_x:i32, start_y:i32, grid: &WrappedGrid<WCell>) -> bool {
    //dump_grid(&grid.grid.borrow());
    let mut x = start_x;
    let y = start_y;
    grid.set(x, y, WCell::WetSand);
    let mut unbound = false;
    // left
    loop {
        match grid.get(x - 1, y) {
            WCell::Clay | WCell::WetSand => { break; },
            WCell::Water => panic!("did not expect to find water"),
            _ => (),
        }
        x -= 1;
        match grid.get(x, y + 1) {
            WCell::Sand | WCell::WetSand =>
                if go_vertical(x, y, &grid) {
                    unbound = true;
                    break;
                },
            WCell::Clay | WCell::Water =>
                grid.set(x, y, WCell::WetSand),
        }
    }
    let minx = x;

    x = start_x;
    // right
    loop {
        match grid.get(x + 1, y) {
            WCell::Clay | WCell::WetSand => { break; },
            WCell::Water => panic!("did not expect to find water"),
            _ => (),
        }
        x += 1;
        match grid.get(x, y + 1) {
            WCell::Sand | WCell::WetSand =>
                if go_vertical(x, y, &grid) {
                    unbound = true;
                    break;
                },
            WCell::Clay | WCell::Water =>
                grid.set(x, y, WCell::WetSand),
        }
    }
    if !unbound {
        for xx in minx ..= x {
            grid.set(xx, y, WCell::Water);
        }
    }
    return unbound;
}

fn dump_grid(grid: &Grid<WCell>) {
    let f = std::fs::File::create("day17-grid").unwrap();
    let mut stream = BufWriter::new(f);
    grid.dump_to_file(&mut stream, |c| match c {
        WCell::Sand => '.',
        WCell::Water => '~',
        WCell::Clay => '#',
        WCell::WetSand => '|',
    });
}

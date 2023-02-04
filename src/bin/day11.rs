use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

fn main() {
    let input: Vec<i64> = read_input::<i64>();
    let serial_no = input[0];

    let mut grid = Grid::new(1, 1, 300, 300, -1);
    for y in 1i64..301 {
        for x in 1i64..301 {
            grid.set(x, y, power_level(x, y, serial_no));
        }
    }

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &Grid<i64>) {
    let (x, y, _) = search(grid, 3, 3);
    println!("Part 1: {},{}", x, y);
}
fn part2(grid: &Grid<i64>) {
    let (x, y, size) = search(grid, 1, 300);
    println!("Part 2: {},{},{}", x, y, size);
}

fn search(grid: &Grid<i64>, min_sizes: i64, max_sizes: i64)
          -> (i64, i64, i64) {
    let mut max_size = 0i64;
    let mut max_max_x = 0i64;
    let mut max_max_y = 0i64;
    let mut max_max_sum = i64::min_value();
    for size in min_sizes .. max_sizes + 1 {
        let mut max_sum = i64::min_value();
        let mut max_x = 0i64;
        let mut max_y = 0i64;
        for y in 1i64..(302-size) {
            for x in 1i64..(302-size) {
                let mut sum = 0;
                for a in 0i64..size {
                    for b in 0i64..size {
                        sum += grid.get(x+a, y+b);
                    }
                }
                if sum > max_sum {
                    max_sum = sum;
                    max_x = x;
                    max_y = y;
                }
            }
        }
        if max_sum > max_max_sum {
            max_max_sum = max_sum;
            max_max_x = max_x;
            max_max_y = max_y;
            max_size = size;
        }
    }
    return (max_max_x, max_max_y, max_size);
}

fn power_level(x: i64, y: i64, serial_no: i64) -> i64 {
    let rack = x + 10;
    let power = (rack * y + serial_no) * rack;
    (power % 1000) / 100 - 5
}

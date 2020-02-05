use std::vec::Vec;
extern crate advent;

fn main() {
    let input: Vec<i32> = advent::read_input::<i32>();
    let serial_no = input[0];

    let mut grid = advent::Grid::new(1, 1, 300, 300);
    for y in 1i32..301 {
        for x in 1i32..301 {
            grid.set(x, y, power_level(x, y, serial_no));
        }
    }

    part1(&grid);
    part2(&grid);
}

fn part1(grid: &advent::Grid) {
    let (x, y, _) = search(grid, 3, 3);
    println!("Part 1: {},{}", x, y);
}
fn part2(grid: &advent::Grid) {
    let (x, y, size) = search(grid, 1, 300);
    println!("Part 2: {},{},{}", x, y, size);
}

fn search(grid: &advent::Grid, min_sizes: i32, max_sizes: i32)
          -> (i32, i32, i32) {
    let mut max_size = 0i32;
    let mut max_max_x = 0i32;
    let mut max_max_y = 0i32;
    let mut max_max_sum = i32::min_value();
    for size in min_sizes .. max_sizes + 1 {
        let mut max_sum = i32::min_value();
        let mut max_x = 0i32;
        let mut max_y = 0i32;
        for y in 1i32..(302-size) {
            for x in 1i32..(302-size) {
                let mut sum = 0;
                for a in 0i32..size {
                    for b in 0i32..size {
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

fn power_level(x: i32, y: i32, serial_no: i32) -> i32 {
    let rack = x + 10;
    let power = (rack * y + serial_no) * rack;
    (power % 1000) / 100 - 5
}

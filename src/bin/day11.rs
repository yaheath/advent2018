use std::vec::Vec;
use itertools::Itertools;
use ya_advent_lib::read::read_input;
use ya_advent_lib::grid::Grid;

fn power_level(x: i64, y: i64, serial_no: i64) -> i64 {
    let rack = x + 10;
    let power = (rack * y + serial_no) * rack;
    (power % 1000) / 100 - 5
}

fn setup(serial_no: i64) -> Grid<i64> {
    let mut grid = Grid::new(1, 1, 300, 300, 0);
    for y in 1..301 {
        for x in 1..301 {
            grid.set(x, y, power_level(x, y, serial_no));
        }
    }
    grid
}

struct SummedAreaTable(Grid<i64>);
impl SummedAreaTable {
    fn from_grid(grid: &Grid<i64>) -> Self {
        let mut sat = grid.clone_without_data(0);
        for y in grid.y_bounds() {
            for x in grid.x_bounds() {
                let v = grid.get(x, y)
                    + sat.get_or_default(x - 1, y, 0)
                    + sat.get_or_default(x, y - 1, 0)
                    - sat.get_or_default(x - 1, y - 1, 0);
                sat.set(x, y, v);
            }
        }
        Self(sat)
    }
    fn area_of(&self, x1: i64, y1: i64, x2: i64, y2: i64) -> i64 {
        self.0.get(x2, y2)
         + self.0.get_or_default(x1 - 1, y1 - 1, 0)
         - self.0.get_or_default(x1 - 1, y2, 0)
         - self.0.get_or_default(x2, y1 - 1, 0)
    }
}

fn search(grid: &Grid<i64>, min_sizes: i64, max_sizes: i64)
          -> (i64, i64, i64) {
    let sat = SummedAreaTable::from_grid(grid);
    (min_sizes ..= max_sizes)
        .map(|size| (1 ..= 301-size)
            .cartesian_product(1 ..= 301-size)
            .map(|(x, y)| (x, y, sat.area_of(x, y, x+size-1, y+size-1)))
            .max_by_key(|(_,_,a)| *a)
            .map(|(x, y, a)| (x, y, a, size))
            .unwrap()
        )
        .max_by_key(|(_,_,a,_)| *a)
        .map(|(x,y,_,s)| (x,y,s))
        .unwrap()
}

fn part1(grid: &Grid<i64>) -> (i64, i64) {
    let (x, y, _) = search(grid, 3, 3);
    (x, y)
}

fn part2(grid: &Grid<i64>) -> (i64, i64, i64) {
    search(grid, 1, 300)
}

fn main() {
    let input: Vec<i64> = read_input::<i64>();
    let grid = setup(input[0]);
    let (x, y) = part1(&grid);
    println!("Part 1: {},{}", x, y);
    let (x, y, size) = part2(&grid);
    println!("Part 2: {},{},{}", x, y, size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_test() {
        let grid = setup(18);
        assert_eq!(part1(&grid), (33,45));
        assert_eq!(part2(&grid), (90,269,16));
        let grid = setup(42);
        assert_eq!(part1(&grid), (21,61));
        assert_eq!(part2(&grid), (232,251,12));
    }
}

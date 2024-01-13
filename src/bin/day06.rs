use std::cmp::{max, min};
use std::vec::Vec;
use ya_advent_lib::read::read_input;
use ya_advent_lib::coords::Coord2D;
use ya_advent_lib::grid::Grid;

const MARGIN:i64 = 50;

fn bothparts(data: &Vec<Coord2D>, test: bool) -> (i64, i64) {
    let min_x: i64 = data.iter().map(|c| c.x)
        .fold(100000, min) - MARGIN;
    let min_y: i64 = data.iter().map(|c| c.y)
        .fold(100000, min) - MARGIN;
    let max_x: i64 = data.iter().map(|c| c.x)
        .fold(0, max) + MARGIN;
    let max_y: i64 = data.iter().map(|c| c.y)
        .fold(0, max) + MARGIN;
    let mut grid = Grid::new(min_x, min_y, max_x, max_y, -1);
    let mut td_grid = Grid::new(min_x, min_y, max_x, max_y, -1);
    for x in min_x .. max_x+1 {
        for y in min_y .. max_y+1 {
            let mut mindex = -1;
            let mut mindist = -1;
            let mut totaldist = 0;
            for (idx, point) in data.iter().enumerate() {
                let dist = (point.x - x).abs() + (point.y - y).abs();
                totaldist += dist;
                if mindist == -1 || mindist > dist {
                    mindist = dist;
                    mindex = idx as i64;
                }
                else if mindist == dist {
                    mindex = -1;
                }
            }
            grid.set(x, y, mindex);
            td_grid.set(x, y, totaldist);
        }
    }

    let mut counts = vec![0i64; data.len()];
    for x in min_x .. max_x+1 {
        let val = grid.get(x, min_y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
        let val = grid.get(x, max_y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
    }
    for y in min_y+1 .. max_y {
        let val = grid.get(min_x, y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
        let val = grid.get(max_x, y);
        if val >= 0 {
            counts[val as usize] = -1;
        }
    }
    for x in min_x+1 .. max_x {
        for y in min_y+1 .. max_y {
            let val = grid.get(x, y);
            if val >= 0 && counts[val as usize] != -1 {
                counts[val as usize] += 1;
            }
        }
    }
    let mut maxarea = 0;
    for c in counts.iter() {
        if *c >= 0 && maxarea < *c {
            maxarea = *c;
        }
    }

    let mut region = 0;
    for d in td_grid.iter() {
        if *d < if test {32} else {10000} {
            region += 1;
        }
    }
    (maxarea, region)
}

fn main() {
    let input = read_input::<Coord2D>();
    let (part1, part2) = bothparts(&input, false);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day06_test() {
        let input: Vec<Coord2D> = test_input(include_str!("day06.testinput"));
        assert_eq!(bothparts(&input, true), (17, 16));
    }
}

use std::vec::Vec;
use advent_lib::read::read_input;
use advent_lib::grid::Grid;

#[derive(Clone, Copy, PartialEq)]
enum Dir {
    N = 0,
    E,
    S,
    W,
}

#[derive(Clone, Copy)]
enum TrackCell {
    Empty,
    Track(Dir, Dir),
    Cross,
}
impl TrackCell {
    fn is_empty(&self) -> bool {
        matches!(*self, TrackCell::Empty)
    }
}

struct Cart {
    x: i64,
    y: i64,
    dir: Dir,
    num_turns: u32,
}
impl Cart {
    fn new(x: i64, y: i64, dir: Dir) -> Self {
        Cart {
            x: x,
            y: y,
            dir: dir,
            num_turns: 0,
        }
    }
    fn turn(&mut self) {
        let m = (self.num_turns % 3) as i64;
        self.num_turns += 1;
        let nd = (self.dir as i64) + (m - 1);
        self.dir = match nd {
           -1 => Dir::W,
            0 => Dir::N,
            1 => Dir::E,
            2 => Dir::S,
            3 => Dir::W,
            4 => Dir::N,
            _ => panic!(),
        };
    }
}

fn search_coll(carts: &mut Vec<Cart>, with: &Cart) -> bool {
    let mut crashed = false;
    for idx in (0 .. carts.len()).rev() {
        if carts[idx].x == with.x && carts[idx].y == with.y {
            carts.remove(idx);
            crashed = true;
        }
    }
    return crashed;
}

fn step(
    grid: &mut Grid<TrackCell>,
    mut carts: &mut Vec<Cart>,
    crash_happened: &mut Option<(i64, i64)>,
) -> bool {
    carts.sort_unstable_by(|a, b|
        if a.y == b.y {
            b.x.cmp(&a.x)
        } else {
            b.y.cmp(&a.y)
        }
    );
    let mut done: Vec<Cart> = Vec::with_capacity(carts.len());
    while !carts.is_empty() {
        let mut c = carts.pop().unwrap();
        match c.dir {
            Dir::N => c.y -= 1,
            Dir::S => c.y += 1,
            Dir::E => c.x += 1,
            Dir::W => c.x -= 1,
        }
        let loc = grid.get(c.x, c.y);
        match loc {
            TrackCell::Track(a, b) => {
                if c.dir != a && c.dir != b {
                    let diff = (c.dir as i64 - a as i64).abs();
                    if diff == 1 || diff == 3 {
                        c.dir = a;
                    } else {
                        c.dir = b;
                    }
                }
            },
            TrackCell::Cross => {
                c.turn();
            },
            TrackCell::Empty => {
                panic!("empty cell {},{}", c.x, c.y);
            },
        }
        let crashed1 = search_coll(&mut carts, &c);
        let crashed2 = search_coll(&mut done, &c);
        if crashed1 || crashed2 {
            if crash_happened.is_none() {
                crash_happened.replace((c.x, c.y));
            }
        }
        else {
            done.push(c);
        }
    }
    carts.append(&mut done);
    carts.len() > 1
}

fn bothparts(data: &[String]) -> ((i64, i64), (i64, i64)) {
    let width = data.iter().map(|s| s.len()).max().unwrap() as i64;
    let height = data.len() as i64;
    let mut grid = Grid::new(0, 0, width-1, height-1, TrackCell::Empty);
    let mut carts: Vec<Cart> = Vec::new();

    for (uy, line) in data.iter().enumerate() {
        let mut lastc = TrackCell::Empty;
        let y = uy as i64;
        for (ux, c) in line.chars().enumerate() {
            let x = ux as i64;
            let cell = match c {
                '-' => TrackCell::Track(Dir::E, Dir::W),
                '|' => TrackCell::Track(Dir::N, Dir::S),
                '/' => {
                    match lastc {
                        TrackCell::Empty => TrackCell::Track(Dir::E, Dir::S),
                        TrackCell::Cross => TrackCell::Track(Dir::W, Dir::N),
                        TrackCell::Track(a, b) => {
                            if a == Dir::E || b == Dir::E {
                                TrackCell::Track(Dir::W, Dir::N)
                            } else {
                                TrackCell::Track(Dir::E, Dir::S)
                            }
                        },
                    }
                },
                '\\' => {
                    match lastc {
                        TrackCell::Empty => TrackCell::Track(Dir::E, Dir::N),
                        TrackCell::Cross => TrackCell::Track(Dir::W, Dir::S),
                        TrackCell::Track(a, b) => {
                            if a == Dir::E || b == Dir::E {
                                TrackCell::Track(Dir::W, Dir::S)
                            } else {
                                TrackCell::Track(Dir::E, Dir::N)
                            }
                        },
                    }
                },
                '^' => {
                    carts.push(Cart::new(x, y, Dir::N));
                    TrackCell::Track(Dir::N, Dir::S)
                },
                'v' => {
                    carts.push(Cart::new(x, y, Dir::S));
                    TrackCell::Track(Dir::N, Dir::S)
                },
                '<' => {
                    carts.push(Cart::new(x, y, Dir::W));
                    TrackCell::Track(Dir::E, Dir::W)
                },
                '>' => {
                    carts.push(Cart::new(x, y, Dir::E));
                    TrackCell::Track(Dir::E, Dir::W)
                },
                '+' => TrackCell::Cross,
                _ => TrackCell::Empty,
            };
            if !cell.is_empty() {
                grid.set(x, y, cell);
            }
            lastc = cell;
        }
    }

    /*
    grid.print(|c| match c {
        TrackCell::Empty => ' ',
        TrackCell::Cross => '+',
        TrackCell::Track(a, b) => {
            if a == Dir::E && b == Dir::W {
                '-'
            } else if a == Dir::N && b == Dir::S {
                '|'
            } else if a == Dir::E && b == Dir::S || a == Dir::W && b == Dir::N {
                '/'
            } else if a == Dir::W && b == Dir::S || a == Dir::E && b == Dir::N {
                '\\'
            } else {
                '?'
            }
        }
    });
    */

    let mut part1: Option<(i64,i64)> = None;
    while step(&mut grid, &mut carts, &mut part1) { }
    let part2 = if carts.is_empty() { (0,0) } else { (carts[0].x, carts[0].y) };

    (part1.unwrap(), part2)
}

fn main() {
    let input = read_input::<String>();
    let (part1, part2) = bothparts(&input);
    println!("Part 1: {},{}", part1.0, part1.1);
    println!("Part 2: {},{}", part2.0, part2.1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use advent_lib::read::test_input;

    #[test]
    fn day13_test() {
        let input: Vec<String> = test_input(include_str!("day13.testinput"));
        let (part1, _) = bothparts(&input);
        assert_eq!(part1, (7, 3));
        let input: Vec<String> = test_input(include_str!("day13.testinput2"));
        let (_, part2) = bothparts(&input);
        assert_eq!(part2, (6,4));
    }
}

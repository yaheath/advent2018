extern crate advent;
use std::collections::HashMap;
use advent::read::read_input;
use advent::grid::Grid;

#[derive(Clone, Copy, Debug, PartialEq)]
enum MapCell {
    Open,
    Trees,
    Lumber,
}

fn main() {
    let data = read_input::<String>();
    let grid = Grid::from_input(&data, MapCell::Open, 1, |c| match c {
        '#' => MapCell::Lumber,
        '|' => MapCell::Trees,
        _ => MapCell::Open,
    });

    part1(&grid);
    part2(&grid);
}

fn part1(g: &Grid<MapCell>) {
    let mut grid = g.clone();
    for _ in 0..10 {
        grid = step(&grid);
    }
    let (w, l) = grid.iter().fold((0usize, 0usize), |(w, l), c| {
        match c {
            MapCell::Lumber => (w, l+1),
            MapCell::Trees => (w+1, l),
            _ => (w, l),
        }
    });

    println!("Part 1: {}", w * l);
}

fn part2(g: &Grid<MapCell>) {
    let mut grid = g.clone();
    let mut table: HashMap<String, usize> = HashMap::new();
    let mut scores: Vec<usize> = Vec::new();
    for iter in 0.. {
        let key:String = grid.iter().map(|c| match c {
            MapCell::Lumber => '#',
            MapCell::Trees => '|',
            MapCell::Open => '.',
        }).collect();
        if table.contains_key(&key) {
            let startidx = table[&key];
            let cyclesize = iter - startidx;
            let ncycles = (1000000000 - startidx) / cyclesize;
            let remainder = 1000000000 - startidx - ncycles * cyclesize;
            println!("Part 2: {}", scores[startidx + remainder]);
            break;
        }
        let (w, l) = grid.iter().fold((0usize, 0usize), |(w, l), c| {
            match c {
                MapCell::Lumber => (w, l+1),
                MapCell::Trees => (w+1, l),
                _ => (w, l),
            }
        });
        let score = w * l;
        scores.push(score);
        table.insert(key, iter);

        grid = step(&grid);
    }
}

fn step(grid: &Grid<MapCell>) -> Grid<MapCell> {
    let mut next = grid.clone_without_data(MapCell::Open);
    for y in 0 .. grid.y_bounds().end - 1 {
        for x in 0 .. grid.x_bounds().end - 1 {
            let mut n_trees = 0;
            let mut n_lumb = 0;
            for a in -1 ..= 1 {
                for b in -1 ..= 1 {
                    if a == 0 && b == 0 { continue; }
                    match grid.get(x+a, y+b) {
                        MapCell::Lumber => n_lumb += 1,
                        MapCell::Trees => n_trees += 1,
                        MapCell::Open => (),
                    }
                }
            }
            next.set(x, y,
                match grid.get(x, y) {
                    MapCell::Open => if n_trees >= 3 {MapCell::Trees} else {MapCell::Open},
                    MapCell::Trees => if n_lumb >= 3 {MapCell::Lumber} else {MapCell::Trees},
                    MapCell::Lumber => if n_lumb > 0 && n_trees > 0 {MapCell::Lumber} else {MapCell::Open},
                }
            )
        }
    }
    next
}

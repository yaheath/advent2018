use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::vec::Vec;
use advent_lib::read::read_input;

#[derive(Debug, PartialEq)]
enum Node<'a> {
    Segment(&'a str),
    Path(Vec<Node<'a>>),
    Branch(Vec<Node<'a>>),
}
impl<'a> Node<'a>{
    fn from_path(path: Vec<Node<'a>>) -> Self {
        if path.len() == 1 {
            if let Node::Segment(s) = path[0] {
                Node::Segment(s)
            } else {
                Node::Path(path)
            }
        } else {
            Node::Path(path)
        }
    }
}

enum Dir { N, E, S, W }
impl Dir {
    fn from_char(c: char) -> Self {
        match c {
            'N' => Dir::N,
            'E' => Dir::E,
            'W' => Dir::W,
            'S' => Dir::S,
            _ => panic!(),
        }
    }
}

#[derive(Clone, Copy)]
struct Room {
    door_n: bool,
    door_e: bool,
    door_w: bool,
    door_s: bool,
}
impl Room {
    fn new() -> Self {
        Room { door_n: false, door_e: false, door_w: false, door_s: false }
    }
}

struct ElfMap {
    data: RefCell<HashMap<(i32, i32), Room>>,
}
impl ElfMap {
    fn new() -> Self {
        let mut r = HashMap::new();
        r.insert((0, 0), Room::new());
        Self { data: RefCell::new(r) }
    }
    fn move_from(&self, point: (i32, i32), dir: Dir) -> (i32, i32) {
        let x = point.0;
        let y = point.1;
        let mut data = self.data.borrow_mut();
        let cell: &mut Room = data.get_mut(&point).unwrap();
        let next = match dir {
            Dir::N => {
                cell.door_n = true;
                (x, y-1)
            },
            Dir::E => {
                cell.door_e = true;
                (x+1, y)
            },
            Dir::W => {
                cell.door_w = true;
                (x-1, y)
            },
            Dir::S => {
                cell.door_s = true;
                (x, y+1)
            },
        };
        let nextcell = data.entry(next).or_insert(Room::new());
        match dir {
            Dir::N => nextcell.door_s = true,
            Dir::S => nextcell.door_n = true,
            Dir::E => nextcell.door_w = true,
            Dir::W => nextcell.door_e = true,
        };
        next
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    point: (i32, i32),
    //path: Vec<(i32, i32)>,
}
impl State {
    fn initial(point: (i32, i32)) -> Self {
        State {
            cost: 0,
            point: point,
            //path: Vec::new(),
        }
    }
    fn next_to(&self, point: (i32, i32)) -> Self {
        //let mut path: Vec<(i32, i32)> = Vec::with_capacity(self.path.len() + 1);
        //for p in self.path.iter() {
        //    path.push(*p);
        //}
        //path.push((x, y));
        State {
            cost: self.cost + 1,
            point: point,
            //path: path,
        }
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| other.point.1.cmp(&self.point.1))
            .then_with(|| other.point.0.cmp(&self.point.0))
            //.then_with(|| other.path[0].1.cmp(&self.path[0].1))
            //.then_with(|| other.path[0].0.cmp(&self.path[0].0))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input: String = read_input::<String>().pop().unwrap();
    let (part1, part2) = bothparts(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn bothparts(input: &str) -> (usize, usize) {
    let tree = parse(&input);
    let map = ElfMap::new();
    traverse(&map, &tree, (0, 0));
    //println!("map has {} rooms", map.data.borrow().len());
    let mapdata = map.data.borrow();
    let mut dists: HashMap<(i32, i32), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    dists.insert((0, 0), 0);
    heap.push(State::initial((0, 0)));

    while let Some(state) = heap.pop() {
        if dists.contains_key(&state.point) && state.cost > dists[&state.point] {
            continue;
        }
        let room = mapdata.get(&state.point).unwrap();
        let mut check = |point: (i32, i32)| {
            let next = state.next_to(point);
            if !dists.contains_key(&point) || next.cost < dists[&point] {
                dists.insert(point, next.cost);
                heap.push(next);
            }
        };
        if room.door_n { check((state.point.0, state.point.1 - 1)); }
        if room.door_e { check((state.point.0 + 1, state.point.1)); }
        if room.door_w { check((state.point.0 - 1, state.point.1)); }
        if room.door_s { check((state.point.0, state.point.1 + 1)); }
    }
    let maxcost = dists.values().max_by_key(|t| *t).unwrap();
    let count = dists.values().filter(|t| **t >= 1000).count();
    (*maxcost, count)
}

fn traverse(map: &ElfMap, node: &Node, start: (i32, i32)) -> HashSet<(i32, i32)> {
    let mut starts = HashSet::new();
    starts.insert(start);
    match node {
        Node::Segment(s) => {
            let mut ends = HashSet::new();
            for start in starts {
                let mut loc = start;
                for c in s.chars() {
                    let dir = Dir::from_char(c);
                    loc = map.move_from(loc, dir);
                }
                ends.insert(loc);
            }
            ends
        },
        Node::Path(p) => {
            for n in p.iter() {
                let mut ends = HashSet::new();
                for start in starts {
                    let nexts = traverse(map, n, start);
                    ends.extend(&nexts);
                }
                starts = ends;
            }
            starts
        },
        Node::Branch(b) => {
            let mut ends = HashSet::new();
            for start in starts {
                for n in b.iter() {
                    let nexts = traverse(map, n, start);
                    ends.extend(&nexts);
                }
            }
            ends
        },
    }
}

fn parse<'a>(input: &'a str) -> Node {
    let mut level = 0;
    let mut start = 0;
    let mut path: Vec<Node<'a>> = Vec::new();
    let mut branch: Vec<Node<'a>> = Vec::new();
    for (idx, c) in input.chars().enumerate() {
        match c {
            '^' => start = idx + 1,
            '(' => {
                if level == 0 {
                    if start < idx {
                        path.push(Node::Segment(&input[start..idx]));
                    }
                    start = idx + 1;
                }
                level += 1;
            },
            ')' => {
                assert_ne!(level, 0);
                level -= 1;
                if level == 0 {
                    path.push(parse(&input[start..idx]));
                    start = idx + 1;
                }
            },
            '|' => if level == 0 {
                if start < idx {
                    path.push(Node::Segment(&input[start..idx]));
                }
                branch.push(Node::from_path(path));
                path = Vec::new();
                start = idx + 1;
            },
            '$' => {
                if start < idx {
                    path.push(Node::Segment(&input[start..idx]));
                }
                start = idx + 1;
            },
            'N' | 'E' | 'W' | 'S' => (),
            _ => panic!("invalid character in input"),
        }
    }
    assert_eq!(level, 0);
    if (&input[start..]).len() > 0 || &input[start-1..start] == "|" {
        path.push(Node::Segment(&input[start..]));
    }
    if path.len() > 0 && branch.len() > 0 {
        branch.push(Node::from_path(path));
        path = Vec::new();
    }
    if branch.len() > 0 {
        Node::Branch(branch)
    } else {
        Node::from_path(path)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn node_parse() {
        if let Node::Segment(foo) = parse("^NEWSNEWS$") {
            assert_eq!(foo, "NEWSNEWS");
        } else {
            assert!(false);
        }

        if let Node::Branch(foo) = parse("NEWS|SWEN|EEE") {
            assert_eq!(foo.len(), 3);
            assert_eq!(foo[0], Node::Segment("NEWS"));
            assert_eq!(foo[1], Node::Segment("SWEN"));
            assert_eq!(foo[2], Node::Segment("EEE"));
        } else {
            assert!(false);
        }

        let foo = parse("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(
            foo,
            Node::Path(vec![
                Node::Segment("ENWWW"),
                Node::Branch(vec![
                    Node::Segment("NEEE"),
                    Node::Path(vec![
                        Node::Segment("SSE"),
                        Node::Branch(vec![
                            Node::Segment("EE"),
                            Node::Segment("N")
                        ])
                    ])
                ])
            ])
        );

        let foo = parse("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        println!("{:?}", foo);
        assert_eq!(
            foo,
            Node::Path(vec![
                Node::Segment("ENNWSWW"),
                Node::Branch(vec![
                    Node::Segment("NEWS"),
                    Node::Segment(""),
                ]),
                Node::Segment("SSSEEN"),
                Node::Branch(vec![
                    Node::Segment("WNSE"),
                    Node::Segment(""),
                ]),
                Node::Segment("EE"),
                Node::Branch(vec![
                    Node::Segment("SWEN"),
                    Node::Segment(""),
                ]),
                Node::Segment("NNN"),
            ])
        );
    }

    #[test]
    fn day20_test() {
        let (part1, _part2) = bothparts("^WNE$");
        assert_eq!(part1, 3);
        let (part1, _part2) = bothparts("^ENWWW(NEEE|SSE(EE|N))$");
        assert_eq!(part1, 10);
        let (part1, _part2) = bothparts("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(part1, 18);
        let (part1, _part2) = bothparts("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$");
        assert_eq!(part1, 18);
        let (part1, _part2) = bothparts("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$");
        assert_eq!(part1, 23);
        let (part1, _part2) = bothparts("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$");
        assert_eq!(part1, 31);
    }
}


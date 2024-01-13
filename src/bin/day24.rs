use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use std::vec::Vec;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_grouped_input;

#[derive(Debug)]
enum Input {
    Group(Group),
    Immune,
    Infection,
}

impl FromStr for Input {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"(\d+) unit.* (\d+) hit points( \(.*\))? with.*does (\d+) (\w+) .*ive (\d+)").unwrap();
        }
        lazy_static! {
            static ref A_RE: Regex = Regex::new(r"\((\w+) to (\w+(?:, \w+)*)(?:; (\w+) to (\w+(?:, \w+)*))?\)").unwrap();
        }
        if s == "Immune System:" { return Ok(Input::Immune); }
        if s == "Infection:" { return Ok(Input::Infection); }
        if let Some(caps) = RE.captures(s) {
            let mut weak_to: HashSet<String> = HashSet::new();
            let mut immune_to: HashSet<String> = HashSet::new();
            let n = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
            let hp = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
            if let Some(attr_cap) = caps.get(3) {
                if let Some(acaps) = A_RE.captures(attr_cap.as_str()) {
                    //println!("{:?}", acaps);
                    for n in [1, 3] {
                        match acaps.get(n) {
                            Some(mode) => {
                                if let Some(lst) = acaps.get(n+1) {
                                    let items = lst.as_str().split(", ").map(|s| s.to_string());
                                    if mode.as_str() == "weak" {
                                        weak_to = items.collect();
                                    } else {
                                        immune_to = items.collect();
                                    }
                                }
                            },
                            None => {},
                        }
                    }
                }
                else {
                    return Err(format!("invalid attr: {}", s))
                }
            }
            let d = caps.get(4).unwrap().as_str().parse::<i64>().unwrap();
            let a = caps.get(5).unwrap().as_str().to_string();
            let i = caps.get(6).unwrap().as_str().parse::<i64>().unwrap();
            Ok(Input::Group(Group {
                id: 0,
                army: Army::Unassigned,
                n_units: n,
                hp: hp,
                dmg: d,
                attack: a,
                init: i,
                weak_to: weak_to,
                immune_to: immune_to,
            }))
        }
        else {
            Err(format!("invalid input: {}", s))
        }
    }
}

fn setup(input: &Vec<Vec<Input>>, immune_boost: i64) -> (Vec<Group>, Vec<Group>) {
    let mut immune = Vec::new();
    let mut infection = Vec::new();
    let mut id = (0..).into_iter();
    for n in 0..2 {
        match input[n][0] {
            Input::Immune => {
                immune = input[n].iter()
                    .skip(1)
                    .map(|ig| match ig {
                        Input::Group(g) => g.assign(id.next().unwrap(), Army::Immune, immune_boost),
                        _ => panic!(),
                    })
                    .collect();
            },
            Input::Infection => {
                infection = input[n].iter()
                    .skip(1)
                    .map(|ig| match ig {
                        Input::Group(g) => g.assign(id.next().unwrap(), Army::Infection, 0),
                        _ => panic!(),
                    })
                    .collect();
            },
            _ => panic!(),
        }
    }
    (immune, infection)
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Army {
    Immune,
    Infection,
    Unassigned,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Group {
    id: usize,
    army: Army,
    n_units: i64,
    hp: i64,
    dmg: i64,
    init: i64,
    attack: String,
    weak_to: HashSet<String>,
    immune_to: HashSet<String>,
}
impl Group {
    fn assign(&self, id: usize, army: Army, boost: i64) -> Self {
        let mut new = self.clone();
        new.id = id;
        new.army = army;
        new.dmg += boost;
        new
    }
    fn ep(&self) -> i64 {
        self.n_units * self.dmg
    }
    fn potential_damage(&self, other: &Self) -> i64 {
        if other.immune_to.contains(&self.attack) {
            return 0;
        }
        if other.weak_to.contains(&self.attack) {
            return self.ep() * 2;
        }
        return self.ep();
    }
}

impl Ord for Group {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ep().cmp(&other.ep())
            .then_with(|| self.init.cmp(&other.init))
    }
}
impl PartialOrd for Group {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn select_targets(groups: &HashMap<usize, Group>) -> HashMap<usize,usize> {
    let mut heap: BinaryHeap<&Group> = BinaryHeap::new();
    groups.iter().for_each(|(_,g)| heap.push(g));
    let mut targetable: HashSet<usize> = HashSet::from_iter(groups.iter().map(|(k,_)| *k));
    let mut targets = HashMap::new();
    while let Some(group) = heap.pop() {
        if let Some(tgt) = targetable.iter()
            .filter(|id| groups[id].army != group.army)
            .map(|id| Reverse(
                (group.potential_damage(&groups[id]), groups[id].ep(), groups[id].init, id)
            ))
            .filter(|rtuple| rtuple.0.0 > 0)
            .sorted()
            .map(|rtuple| *(rtuple.0.3))
            .next() {
                targets.insert(group.id, tgt);
                targetable.remove(&tgt);
        }
    }
    targets
}

fn do_attacks(groups: &mut HashMap<usize, Group>, targets: &HashMap<usize, usize>) -> bool {
    let mut kills = false;
    let ids_by_initiative = Vec::from_iter(
        targets.iter()
            .map(|(id,_)| id)
            .sorted_by_key(|id| Reverse(groups[id].init))
    );
    for id in ids_by_initiative {
        let dmg = groups[id].potential_damage(&groups[&targets[id]]);
        if dmg > 0 {
            let victim = groups.get_mut(&targets[id]).unwrap();
            let n_kills = dmg / victim.hp;
            if n_kills > 0 {
                victim.n_units -= n_kills;
                if victim.n_units < 0 { victim.n_units = 0; }
                kills = true;
            }
        }
    }
    kills
}

fn fight(immune: Vec<Group>, infection: Vec<Group>) -> (Vec<Group>, Vec<Group>, bool) {
    let mut groups = HashMap::from_iter(
        immune.into_iter()
            .chain(infection.into_iter())
            .map(|g| (g.id, g))
    );

    let targets = select_targets(&groups);
    let kills = do_attacks(&mut groups, &targets);

    let mut immune = Vec::new();
    let mut infection = Vec::new();
    groups.into_iter()
        .filter(|(_,g)| g.n_units > 0)
        .for_each(|(_,g)| {
            match g.army {
                Army::Immune => immune.push(g),
                Army::Infection => infection.push(g),
                Army::Unassigned => panic!(),
            }
        });
    (immune, infection, kills)
}

fn combat(input: &Vec<Vec<Input>>, immune_boost: i64) -> (i64, Army) {
    let (mut immune, mut infection) = setup(&input, immune_boost);
    let mut kills;
    loop {
        (immune, infection, kills) = fight(immune, infection);
        if immune.len() == 0 || infection.len() == 0 || !kills {
            break;
        }
    }
    let victor = if immune.len() == 0 {
        Some(infection)
    } else if infection.len() == 0 {
        Some(immune)
    } else {
        None
    };
    if let Some(v) = victor {
        let n_units:i64 = v.iter().map(|g| g.n_units).sum();
        (n_units, v[0].army)
    } else {
        (0, Army::Unassigned)
    }
}

fn part1(input: &Vec<Vec<Input>>) -> i64 {
    let (answer, _) = combat(input, 0);
    answer
}

fn part2(input: &Vec<Vec<Input>>) -> i64 {
    // binary search would work a lot better here
    for b in 1.. {
        let (units, victor) = combat(input, b);
        if victor == Army::Immune {
            return units;
        }
    }
    unreachable!();
}

fn main() {
    let input = read_grouped_input::<Input>();
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}


#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::grouped_test_input;

    #[test]
    fn day24_test() {
        let input: Vec<Vec<Input>> = grouped_test_input(include_str!("day24.testinput"));
        assert_eq!(part1(&input), 5216);
        assert_eq!(part2(&input), 51);
    }
}

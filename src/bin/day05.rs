use std::vec::Vec;
use lazy_static::lazy_static;
use regex::Regex;
use ya_advent_lib::read::read_input;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Aa|aA|Bb|bB|Cc|cC|Dd|dD|Ee|eE|Ff|fF|Gg|gG|Hh|hH|Ii|iI|Jj|jJ|Kk|kK|Ll|lL|Mm|mM|Nn|nN|Oo|oO|Pp|pP|Qq|qQ|Rr|rR|Ss|sS|Tt|tT|Uu|uU|Vv|vV|Ww|wW|Xx|xX|Yy|yY|Zz|zZ").unwrap();
}

fn react(input: &str) -> String {
    let mut polymer = Box::new(input.to_owned());
    loop {
        let np = RE.replace_all(&polymer, "").to_string();
        if np == *polymer { break; }
        polymer = Box::new(np.to_owned());
    }
    return *polymer;
}

fn part1(input: &str) -> usize {
    let polymer = react(input);
    polymer.len()
}

fn part2(input: &str) -> usize {
    let mut minlen = input.len();
    for c in b'a'..=b'z' {
        let mut pat = String::from("(?i)");
        pat.push(c as char);
        let reg = Regex::new(&pat).unwrap();
        let size = react(&reg.replace_all(input, "").to_string()).len();
        if size < minlen {
            minlen = size;
        }
    }
    minlen
}

fn main() {
    let input: Vec<String> = read_input::<String>();
    println!("Part 1: {}", part1(&input[0]));
    println!("Part 2: {}", part2(&input[0]));
}

#[cfg(test)]
mod tests {
    use super::*;
    use ya_advent_lib::read::test_input;

    #[test]
    fn day05_test() {
        let input: Vec<String> = test_input("dabAcCaCBAcCcaDA".into());
        assert_eq!(part1(&input[0]), 10);
        assert_eq!(part2(&input[0]), 4);
    }
}

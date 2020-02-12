#[macro_use] extern crate lazy_static;
use regex::Regex;
use std::vec::Vec;
extern crate advent;
use advent::read::read_input;

fn main() {
    let input: Vec<String> = read_input::<String>();
    part1(&input[0]);
    part2(&input[0]);
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"Aa|aA|Bb|bB|Cc|cC|Dd|dD|Ee|eE|Ff|fF|Gg|gG|Hh|hH|Ii|iI|Jj|jJ|Kk|kK|Ll|lL|Mm|mM|Nn|nN|Oo|oO|Pp|pP|Qq|qQ|Rr|rR|Ss|sS|Tt|tT|Uu|uU|Vv|vV|Ww|wW|Xx|xX|Yy|yY|Zz|zZ").unwrap();
}

fn react(input: &String) -> String {
    let mut polymer = Box::new(input.clone());
    loop {
        let np = RE.replace_all(&polymer, "").to_string();
        if np == *polymer { break; }
        polymer = Box::new(np);
    }
    return *polymer;
}

fn part1(input: &String) {
    let polymer = react(input);
    println!("Part 1: {}", polymer.len());
}

fn part2(input: &String) {
    let mut minlen = input.len();
    for c in b'a'..b'{' {
        let mut pat = String::from("(?i)");
        pat.push(c as char);
        let reg = Regex::new(&pat).unwrap();
        let size = react(&reg.replace_all(input, "").to_string()).len();
        if size < minlen {
            minlen = size;
        }
    }
    println!("Part 2: {}", minlen);
}

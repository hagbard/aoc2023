use std::str::FromStr;

use lazy_regex::regex_captures;

use rpoly::RPoly;

use crate::rpoly;
use crate::xy::Dir;
use crate::xy::Dir::{Down, Left, Right, Up};

pub fn run(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    (parse_poly(&lines, &parse1).get_external_area(),
     parse_poly(&lines, &parse2).get_external_area())
}

fn parse1(s: &str) -> (Dir, i32) {
    let (_, dstr, lstr) = regex_captures!(r"([UDLR]) (\d+) \(#[0-9a-f]+\)", &s).unwrap();
    let len = i32::from_str(lstr).unwrap();
    match dstr {
        "R" => (Right, len),
        "D" => (Up, len),
        "L" => (Left, len),
        "U" => (Down, len),
        _ => panic!("Bad input")
    }
}

fn parse2(s: &str) -> (Dir, i32) {
    let (_, hstr, dstr) = regex_captures!(r"[UDLR] \d+ \(#([0-9a-f]{5})([0-3])\)", &s).unwrap();
    let len = i32::from_str_radix(hstr, 16).unwrap();
    match dstr {
        "0" => (Right, len),
        "1" => (Up, len),
        "2" => (Left, len),
        "3" => (Down, len),
        _ => panic!("Bad input")
    }
}

fn parse_poly(lines: &[&str], parse: &dyn Fn(&str) -> (Dir, i32)) -> RPoly {
    let mut poly = RPoly::new();
    for &s in lines {
        let (dir, len) = parse(s);
        poly.add_relative(dir, len);
    }
    poly
}

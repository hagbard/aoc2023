use std::collections::HashMap;
use std::string::ToString;

use aho_corasick::AhoCorasick;
use lazy_static::lazy_static;
use strum::IntoEnumIterator;

use crate::util::Digits;

lazy_static! {
    // Find names and numerical values as sub-strings.
    static ref MATCHER: AhoCorasick = AhoCorasick::new(itertools::chain(
        Digits::iter().map(|d| d.to_string().to_ascii_lowercase()),
        Digits::iter().map(|d| (d as i32).to_string()))).unwrap();
}

lazy_static! {
    // Map names and string values to integers.
    static ref DIGIT_MAP: HashMap<String, u32> = HashMap::from_iter(itertools::chain(
        Digits::iter().map(|d| (d.to_string().to_ascii_lowercase(), d as u32)),
        Digits::iter().map(|d| d as u32).map(|d| (d.to_string(), d))));
}

pub fn run(input: &str) -> (u32, u32) {
    let lines: Vec<&str> = input.lines().collect();
    (lines.iter().map(|s| 10 * ldigit(s) + rdigit(s)).sum::<u32>(),
     lines.iter().map(|s| num(s)).sum::<u32>())
}

fn ldigit(s: &str) -> u32 { s.chars().find(char::is_ascii_digit).and_then(|c| c.to_digit(10)).unwrap() }

fn rdigit(s: &str) -> u32 { s.chars().rfind(char::is_ascii_digit).and_then(|c| c.to_digit(10)).unwrap() }

fn num(s: &str) -> u32 {
    let mut it = MATCHER.find_overlapping_iter(s);
    let first = &s[it.next().unwrap().span()];
    // Messier because the default value is the &str, not the Match object. ¯\_(ツ)_/¯
    let last = it.last().map(|m| &s[m.span()]).unwrap_or(first);
    10 * DIGIT_MAP.get(first).unwrap() + DIGIT_MAP.get(last).unwrap()
}

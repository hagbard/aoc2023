use std::str::FromStr;

use itertools::Itertools;
use lazy_regex::regex_captures;

pub fn run(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();
    (solve(&lines, false), solve(&lines, true))
}

fn solve(lines: &[&str], is_wild: bool) -> usize {
    lines.iter()
        .map(|&s| regex_captures!(r"(\S+) (\d+)", s).unwrap())
        .map(|(_, hand, bid)| (sort_key(hand, is_wild), usize::from_str(bid).unwrap()))
        .sorted()
        .map(|(_, n)| n)
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

const CARD_RANKING: &str = "..23456789TJQKA";
const WILDCARD_RANKING: &str = ".J23456789T.QKA";

fn sort_key(s: &str, is_wild: bool) -> usize {
    // Start with hand string (e.g. "KT9KK"
    let mut primary_key = s.chars()
        // Ignore Jokers if we're in wild-card mode.
        .filter(|&c| !is_wild || c != 'J')
        // Group same chars together (e.g. "9KKKT").
        .sorted()
        // Emit the count of each run of the same char (e.g. [1, 3, 1]).
        .group_by(|&c| c).into_iter().map(|(_, g)| g.count())
        // Sort in reverse order (e.g. [3, 1, 1]).
        .sorted_by(|a, b| b.cmp(a))
        // Add 1 extra zero for special case of 5-of-a-kind.
        .chain([0])
        // Take only first 2 (e.g. [3, 1], or [5, 0] for special case).
        .take(2)
        // And reduce to a simple hex value (e.g. 0x31) which is a unique sort key per hand type.
        .reduce(|m, n| (16 * m) + n)
        .unwrap();
    // Card ranking provides secondary sort key based on the index of card's char.
    let mut rank = CARD_RANKING;
    if is_wild {
        // In wildcard mode, the number of Jokers is just added to whatever card is most frequent.
        // It's even possible there were 5 Jokers, so the initial key was zero, but this still works.
        primary_key += 16 * s.chars().filter(|&c| c == 'J').count();
        // The ranking is also changed so Jokers are lowest.
        rank = WILDCARD_RANKING;
    }
    let secondary_key =
        s.chars().map(|c| rank.find(c).unwrap()).reduce(|m, n| (16 * m) + n).unwrap();
    (primary_key << 20) + secondary_key
}
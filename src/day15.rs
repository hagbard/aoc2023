use std::collections::HashMap;
use std::str::FromStr;

use itertools::Itertools;
use lazy_regex::regex_captures;

pub fn run(input: &str) -> (u32, u32) {
    let mut part1: u32 = 0;
    let mut map: HashMap<u32, Vec<(String, u32)>> = HashMap::new();

    for inst in input.split(',') {
        part1 += hash(inst);
        let (_, label, op, nstr) = regex_captures!(r"([a-z]+)([=-])(\d+)?", inst).unwrap();
        let values = map.entry(hash(label)).or_insert(Vec::new());

        let pos = values.iter().find_position(|(l, _)| l == label).map(|(i, _)| i);

        if op == "-" {
            if let Some(i) = pos {
                values.remove(i);
            }
        } else {
            let value = u32::from_str(nstr).unwrap();
            if let Some(i) = pos {
                values[i].1 = value;
            } else {
                values.push((label.to_string(), value));
            }
        }
    }
    (part1, map.iter().map(|(&h, v)| (h + 1) * sum_vec(v)).sum())
}

fn hash(s: &str) -> u32 {
    s.chars().fold(0u32, |h, c| (17 * (h + (c as u32))) & 0xFF)
}

fn sum_vec(v: &[(String, u32)]) -> u32 {
    v.iter().enumerate().map(|(i, &(_, n))| (i as u32 + 1) * n).sum()
}

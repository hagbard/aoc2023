use std::collections::HashSet;

use lazy_regex::regex_captures;

pub fn run(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();

    let mut mul = Vec::from([1i32].repeat(lines.len()));
    let mut part1 = 0;
    let mut part2 = 0;
    for (i, score) in lines.iter()
        .map(|&s| regex_captures!(r"Card\s+\d+:\s+([\d ]+) \| ([\d ]+)", s).unwrap())
        .map(|(_, w, h)| as_set(w).intersection(&as_set(h)).count())
        .enumerate() {
        //
        part1 += (1 << score) / 2;
        let xtra = mul[i];
        part2 += xtra;
        let _ = &mul[(i + 1)..].iter_mut().take(score).for_each(|m| *m += xtra);
    }
    (part1, part2)
}

fn as_set(s: &str) -> HashSet<u32> {
    HashSet::from_iter(s.split_whitespace().map(|n| u32::from_str_radix(n, 10).unwrap()))
}
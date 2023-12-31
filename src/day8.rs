use std::collections::HashMap;

use lazy_regex::regex_captures;
use num_integer::Integer;

pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<&str> = input.lines().collect();
    let dirn = &lines[0];
    let dirn_count = dirn.len() as u32;

    let mut net: HashMap<u32, Out> = HashMap::new();
    for &line in &lines[2..] {
        let (_, node, lhs, rhs) =
            regex_captures!(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)", line).unwrap();
        net.insert(parse(node), Out { left: parse(lhs), right: parse(rhs) });
    }

    let mut cur = parse("AAA");
    let mut part1: u32 = 0;
    let end = parse("ZZZ");
    for d in dirn.chars().cycle() {
        part1 += 1;
        cur = net.get(&cur).unwrap().get(d);
        if cur == end { break; }
    }

    // (steps-to-end-state, cycle-length)
    let mut loops: Vec<(u32, u32)> = vec![];
    for start in net.keys().filter(|&n| ends_with('A', n)) {
        let mut trail: HashMap<Locn, u32> = HashMap::new();
        let mut end: Option<u32> = None;
        let mut count = 0;

        let mut cur = *start;
        for d in dirn.chars().cycle() {
            count += 1;
            cur = net.get(&cur).unwrap().get(d);
            let loc = Locn { state: cur, index: count % dirn_count };
            if let Some(&prev_count) = trail.get(&loc) {
                // Assume we saw an end state before we looped.
                loops.push((end.unwrap(), count - prev_count));
                break;
            }
            trail.insert(loc, count);
            if ends_with('Z', &cur) { end = Some(count); }
        }
    }
    // The offset in the input cycle at which we end (this is the same for all "ghosts"
    // since they work in lock step). If this is NOT zero, there's a *lot* more work to
    // be done to determine the total step count. Probably by design, this *is* zero,
    // but there is NO requirement for this in the general case according to the rules !!
    //
    // If adjust == 0, then the result is exactly N input cycles, where N is the
    // least-common-multiple of the number of cycles for each ghost.
    let adjust = loops[0].0 % dirn_count;
    assert_eq!(adjust, 0, "Unexpected state");

    let part2: u64 = dirn_count as u64 *
        loops.iter()
            .map(|&(_, cycle_length)| (cycle_length / dirn_count) as u64)
            .reduce(|a, b| a.lcm(&b)).unwrap();

    (part1 as u64, part2)
}

fn parse(s: &str) -> u32 {
    s.chars().map(|c| c as u32).reduce(|m, c| (m << 8) + c).unwrap()
}

fn ends_with(c: char, state: &u32) -> bool {
    (state & 0xFF) == c as u32
}

#[derive(Debug, Copy, Clone)]
struct Out {
    left: u32,
    right: u32,
}

impl Out {
    fn get(&self, d: char) -> u32 {
        if d == 'L' { self.left } else { self.right }
    }
}

// Location in cycle, used to detect repeated state.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Locn {
    state: u32,
    index: u32,
}

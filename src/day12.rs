use std::collections::{HashMap, HashSet};
use std::str::FromStr;

use itertools::Itertools;
use lazy_regex::regex_captures;

// Python: https://www.online-python.com/wIkV8ydE5s
pub fn run(input: &str) -> (u64, u64) {
    let lines: Vec<(&str, Vec<usize>)> =
        input.lines()
            .map(|s| regex_captures!(r"(\S+) (\S+)", s).unwrap())
            .map(|(_, lhs, rhs)| (lhs, rhs.split(',').map(atou).collect()))
            .collect();

    (lines.iter().map(|(r, g)| Solver::solve(r, g, 1)).sum::<u64>(),
     lines.iter().map(|(r, g)| Solver::solve(r, g, 5)).sum::<u64>())
}

fn atou(s: &str) -> usize {
    usize::from_str(s).unwrap()
}

#[derive(Debug)]
struct Transition {
    // Map candidate-group length to offset from end of current group.
    // Contains special entry of '0' if the group cannot fit at any future position in the record
    // (this lets us early exit rather than fruitlessly attempting later offsets).
    out_map: HashMap<usize, usize>,
    can_advance: bool,
}

impl Transition {
    fn new(record: &str, group_lengths: &HashSet<usize>) -> Transition {
        let out_map: HashMap<usize, usize> =
            group_lengths.iter()
                .filter(|&glen| Transition::can_accept(record, *glen))
                .map(|&glen| (glen, Transition::find_skip(&record[glen..])))
                .collect();
        Transition { out_map, can_advance: &record[0..1] != "#" }
    }

    fn accept_group(&self, pos: usize, group_length: usize) -> Option<usize> {
        if let Some(&skip) = self.out_map.get(&group_length) {
            if skip > 0 { Some(pos + group_length + skip) } else { Some(0) }
        } else {
            None
        }
    }

    // Whether we can accept group at the given length at the start of the record slice.
    fn can_accept(remaining: &str, glen: usize) -> bool {
        let max_length = remaining.find('.').unwrap_or(remaining.len());
        // Group fits at the start of the record, and if it's shorter than the record it is not followed by '#'.
        glen <= max_length && (glen == remaining.len() || &remaining[glen..glen + 1] != "#")
    }

    fn find_skip(group_end: &str) -> usize {
        group_end.chars().tuple_windows().enumerate()
            // Next start is when previous isn't '#' and current isn't '.'.
            .skip_while(|(_, (p, c))| *p == '#' || *c == '.')
            // Add 1 to reference the index of the "current" char above".
            .map(|(i, _)| i + 1)
            .next()
            .unwrap_or(0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Key {
    pos: usize,
    group_index: usize,
}

#[derive(Debug)]
struct Solver {
    record: String,
    groups: Vec<usize>,
    transition_map: HashMap<usize, Transition>,
    counts: HashMap<Key, u64>,
    min_length: usize,
}

impl Solver {
    fn solve(record: &str, groups: &Vec<usize>, factor: usize) -> u64 {
        let mut solver = Solver::new(record, groups, factor);
        solver.count_recursively(&Key { pos: 0, group_index: 0 })
    }

    fn new(one_record: &str, groups: &Vec<usize>, factor: usize) -> Solver {
        let record: String = [one_record].repeat(factor).join("?");
        let gset: HashSet<usize> = HashSet::from_iter(groups.iter().map(|l| *l));
        let transition_map = HashMap::from_iter(
            (0..record.len()).map(|p| (p, Transition::new(&record[p..], &gset))));
        let min_length = record.rfind('#').map(|n| n + 1).unwrap_or(0);
        Solver { record, groups: groups.repeat(factor), transition_map, counts: HashMap::new(), min_length }
    }

    fn count_recursively(&mut self, key: &Key) -> u64 {
        if let Some(cached_count) = self.counts.get(key) {
            return *cached_count;
        }
        let group_length = self.groups[key.group_index];
        let next_index = key.group_index + 1;
        let mut count: u64 = 0;
        let mut pos = key.pos;
        while pos < self.record.len() {
            let transition = &self.transition_map[&pos];
            // Resolve *before* attempting mutable borrow for recursion since 'transition' is a
            // reference inside the immutable borrow of the 'in_map'.
            let can_advance = transition.can_advance;
            if let Some(next_pos) = transition.accept_group(pos, group_length) {
                if next_index == self.groups.len() {
                    if pos + group_length >= self.min_length { count += 1; }
                } else if next_pos > 0 {
                    count += self.count_recursively(&Key { pos: next_pos, group_index: next_index });
                } else {
                    break;
                }
            }
            if !can_advance {
                break;
            }
            pos += 1;
        }
        self.counts.insert(*key, count);
        return count;
    }
}

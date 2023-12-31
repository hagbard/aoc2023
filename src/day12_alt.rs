use std::str::FromStr;

use lazy_regex::regex_captures;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rustc_hash::FxHashMap;

// https://adventofcode.com/2023/day/12
// https://www.online-python.com/Z8Mv6FPX1f
pub fn run(input: &str) -> (u64, u64) {
    let records: Vec<_> = input.lines()
        .map(|s| regex_captures!(r"(\S+) (\S+)", s).unwrap())
        .map(|(_, lhs, rhs)| (lhs, to_groups(rhs))).collect();
    // Parallel iteration brings it down from 30ms to 10ms.
    (records.par_iter().map(|(r, g)| solve(*r, g)).sum::<u64>(),
     records.par_iter().map(|(r, g)| solve(&[*r].repeat(5).join("?"), &g.repeat(5))).sum::<u64>())
}

fn solve(record: &str, groups: &Vec<u32>) -> u64 {
    let last_group = groups.len() - 1;
    assert!(last_group < 0x8000, "too many groups");
    assert!(*groups.iter().max().unwrap() < 0x8000, "groups too large");

    // Note: Using state encoded as u32 with the FxHashMap is a significant speed improvement.
    let final_state = state(last_group, groups[last_group]);

    // The last position of a mandatory part. Fail if all groups are placed before this.
    let min_last_index = record.rfind('#').unwrap_or(0);

    // Initially there is one way to get to the initial state (group=0, nothing consumed).
    let mut states: FxHashMap<u32, u64> = FxHashMap::default();
    states.insert(state(0, 0), 1);

    let mut total: u64 = 0;
    for (pos, chr) in record.chars().enumerate() {
        // Create the map for the next set of states. Typically between 5 and 50 states exist at
        // each step, some of which are propagated to the next cycle, while others are abandoned.
        // Note: Possible performance improvement if we can avoid swapping the entire table.
        let mut next_states: FxHashMap<u32, u64> = FxHashMap::default();

        // Loop through the current set of states (up to 50 or so items) and propagate or abandon
        // them as appropriate with respect to the currrent input character.
        //
        // When the input is ambiguous (i.e. '?'), an empty group will be propaagate twice, once
        // by consuming a character into the group, and once by allowing the start position of
        // the group to be advanced. This is the only mechanism for creating multiplicity in the
        // counting of states, and yet it results in billions of states for some input.
        //
        // Loop invariant: current.group_index < groups.len().
        for (&current, &count) in states.iter() {
            // Get the length of the current group we are handling.
            let group_length = groups[group_index(current)];

            // Test for input which can lie between groups. Only empty/complete groups can be
            // propagated by this section. Partially completed groups are dropped at this step.
            if chr == '.' || chr == '?' {
                if consumed(current) == 0 {
                    // Already empty groups survive unchanged when nothing need be consumed.
                    *next_states.entry(current).or_insert(0) += count;
                } else if consumed(current) == group_length {
                    // Completed groups (which must have been added in the previous round) are only
                    // promoted to the next empty group if the current character (the one
                    // immediately after the end of the group) is NOT a '#'.
                    *next_states.entry(next_group(current)).or_insert(0) += count;
                }
            }

            // Test for input which can be consumed by the current (incomplete) group. Completed
            // groups are handled above, this is for attempting to consume the current character
            // in an incomplete (but possibly empty) group.
            if (chr == '#' || chr == '?') && consumed(current) < group_length {
                let next = consume(current);
                if next != final_state {
                    // This may now be a completed group. This is a transient state which gets
                    // turned into an empty group in the next iteration (if the next character
                    // is not a '#').
                    *next_states.entry(next).or_insert(0) += count;
                } else {
                    // Once the final state is reached there's no need to retain this state, and
                    // we can just increment our total. However we only count the total if we have
                    // consumed the final '#'.
                    total += if pos >= min_last_index { count } else { 0 };
                }
            }
        }
        states = next_states;
    }
    return total;
}

// ---- Encoding state `(group-index, consumed-count)` in a u32 for performance ----
#[inline]
fn state(group_index: usize, consumed: u32) -> u32 {
    ((group_index as u32) << 16) + consumed
}

#[inline]
fn next_group(state: u32) -> u32 {
    (state & 0xffff0000) + 0x10000
}

#[inline]
fn consume(state: u32) -> u32 {
    state + 1
}

#[inline]
fn group_index(state: u32) -> usize {
    (state >> 16) as usize
}

#[inline]
fn consumed(state: u32) -> u32 {
    state & 0xffff
}

fn to_groups(s: &str) -> Vec<u32> { s.split(',').map(atou).collect::<Vec<_>>() }

fn atou(s: &str) -> u32 { u32::from_str(s).unwrap() }
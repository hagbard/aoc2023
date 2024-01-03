use std::fmt::Debug;

use itertools::Itertools;

use crate::agrid::AGrid;

// https://adventofcode.com/2023/day/13
pub fn run(input: &str) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    for img in input.split("\n\n").map(Img::new) {
        part1 += flip_offset(&img.cols) + (100 * flip_offset(&img.rows));
        part2 += corrected_flip_offset(&img.cols) + (100 * corrected_flip_offset(&img.rows));
    }
    (part1, part2)
}

fn flip_offset(arr: &[u32]) -> usize {
    // Without accounting for correction, the flip index is just the unique index for which
    // is_flip() returns true (note that the flip index is never 0 since it's "to the right
    // of the mirror").
    (1..arr.len()).filter(|&i| is_flip(arr, i, None)).at_most_one().unwrap().unwrap_or(0)
}

fn corrected_flip_offset(arr: &[u32]) -> usize {
    // Map all pairs of (even x odd) indices, since mirrored elements always have opposite parity.
    (0..arr.len()).step_by(2)
        .flat_map(|i| (1..arr.len()).step_by(2).map(move |j| (i, j)))
        // Keep only those index pairs whose values differ by exactly one bit.
        .filter(|&(a, b)| one_bit_diff(arr[a], arr[b]))
        // Map to the mid-point "flip index" and positive offset to corrected values.
        .map(to_index_and_offset)
        // Ensure that the correction is valid (many are not).
        .filter(|e| is_flip(arr, e.0, Some(e.1)))
        // Keep only the flip index.
        .map(|e| e.0)
        // And expect at most one result (returning 0 if no result).
        .at_most_one().unwrap().unwrap_or(0)
}

//  [0...idx-1] <-> [idx...arr.len()-1]
//  |   idx   |  +  |  arr.len()-idx  | == arr.len()
fn is_flip(arr: &[u32], flip_idx: usize, correction_offset: Option<usize>) -> bool {
    // Shortest number of elements to either edge forms the mirrored region.
    let len = flip_idx.min(arr.len() - flip_idx);
    // If we were given a correction offset, it must be in the mirrored region.
    if correction_offset.is_some_and(|n| n >= len) { return false; }
    // Test the offsets either side to the flip index.
    (0..len)
        // Values at the correction offset are assumed to be valid without checking.
        .filter(|&i| correction_offset.map(|n| n != i).unwrap_or(true))
        // And all remaining offsets must result in mirrored values being equal.
        .all(|i| arr[flip_idx - (i + 1)] == arr[flip_idx + i])
}

fn one_bit_diff(a: u32, b: u32) -> bool {
    let xor = a ^ b;
    xor != 0 && (xor & (xor - 1)) == 0
}

// Convert a pair of even/odd error indices where a correction can occur,
// to the assumed flip index and relative correction offset.
fn to_index_and_offset(err: (usize, usize)) -> (usize, usize) {
    // Flip index is "to the right" of the mirror (i.e. rounded up).
    let flip_idx = (err.0 + err.1 + 1) / 2;
    // The offset is just the distance from the flip index to the largest error index.
    let offset = err.0.max(err.1) - flip_idx;
    (flip_idx, offset)
}

#[derive(Debug)]
struct Img {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Img {
    fn new(s: &str) -> Img {
        let grid = AGrid::from_lines(s);
        let mut cols: Vec<u32> = [0u32].repeat(grid.width());
        let mut rows: Vec<u32> = [0u32].repeat(grid.height());
        for p in grid.all_points() {
            if p.chr == '#' {
                cols[p.pos.x] |= 1 << p.pos.y;
                rows[p.pos.y] |= 1 << p.pos.x;
            }
        }
        Img { rows, cols }
    }
}
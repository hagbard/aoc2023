use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use crate::agrid::AGrid;

use crate::xy::Point;

pub fn run(input: &str) -> (i64, i64) {
    let grid: AGrid = input.lines().collect();
    let galaxies: HashSet<Point<usize>> =
        grid.all_points().filter_map(|p| if p.chr == '#' { Some(p.pos) } else { None }).collect();
    (sum_dist(&galaxies, 2), sum_dist(&galaxies, 1_000_000))
}

fn sum_dist(galaxies: &HashSet<Point<usize>>, factor: usize) -> i64 {
    let xmap = expand(&galaxies.iter().map(|p| p.x).collect(), factor);
    let ymap = expand(&galaxies.iter().map(|p| p.y).collect(), factor);
    galaxies.iter()
        .map(|p| Point::new(*xmap.get(&p.x).unwrap(), *ymap.get(&p.y).unwrap()))
        .tuple_combinations()
        .map(|(a, b)| (a.x - b.x).abs() + (a.y - b.y).abs())
        .sum()
}

fn expand(pset: &HashSet<usize>, factor: usize) -> HashMap<usize, i64> {
    // sorted p = [0, 1, 3, 4, 7, 8, 15]
    // -> p-i = (0, 0, 1, 1, 3, 3, 9)
    // -> i + factor * (p - i)
    // -> {0:0, 1:1, 3:2+f*1, 4:3+f*1, 7:4+f*3, 8:5+f*3, 15:6+f*9}
    HashMap::from_iter(pset.iter().sorted().enumerate().map(|(i, &p)| (p, (i + (factor * (p - i))) as i64)))
}

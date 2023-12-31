use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

use Dir::{Down, Left, Right, Up};

use crate::agrid::{AGrid, GPoint};
use crate::rpoly::RPoly;
use crate::xy::{Dir, Point};

lazy_static!(
    static ref TURN_MAP: HashMap<(Dir, char), Dir> = HashMap::from([
        ((Right, 'J'), Up), ((Right, '7'), Down),
        ((Down, 'L'), Right), ((Down, 'J'), Left),
        ((Left, 'F'), Down), ((Left, 'L'), Up),
        ((Up, '7'), Left), ((Up, 'F'),  Right),
    ]);
);

fn is_straight(c: char, d: Dir) -> bool {
    if d == Left || d == Right { c == '-' } else { c == '|' }
}

fn next_corner_or_start(grid: &AGrid, p: &Point<usize>, d: Dir) -> Option<GPoint> {
    grid.points_after(&p, d).skip_while(|p| is_straight(p.chr, d)).next()
}

pub fn run(input: &str) -> (i64, i64) {
    let grid: AGrid = input.lines().collect();

    let start =
        grid.all_points().filter(|p| p.chr == 'S').exactly_one().unwrap();
    let mut poly = RPoly::new();
    'outer: for d_start in [Right, Down, Left, Up] {
        if let Some(mut cur) = next_corner_or_start(&grid, &start.pos, d_start) {
            if !TURN_MAP.contains_key(&(d_start, cur.chr)) { continue; }

            let mut d_in = d_start;
            loop {
                poly.add_xy(cur.pos.x as i32, cur.pos.y as i32);
                d_in = TURN_MAP[&(d_in, grid.get(&cur.pos))];
                cur = next_corner_or_start(&grid, &cur.pos, d_in).unwrap();
                if cur.chr == 'S' {
                    // Add the start point if it was an implicit corner.
                    if d_in != d_start { poly.add_xy(cur.pos.x as i32, cur.pos.y as i32); }
                    break 'outer;
                }
            }
        }
    }
    (poly.get_perimeter() / 2, poly.get_internal_area())
}
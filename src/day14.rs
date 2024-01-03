// https://www.online-python.com/J1Ork8XmPf


use std::collections::HashMap;

use crate::agrid::AGrid;
use crate::day14::CardinalFrame::{East, North, South, West};
use crate::xy::Point;

// A map of non-rolling rock positions according to a specific direction ID.
// By storing columns of sorted stationary rock locations, we can implement an easy "tilt" algorithm:
//
// Dir of tilt: --->
// Y: (lo) "...###....#....##..." (hi)
//             v      v    v
// Rock pos:   3      10   15
// Store the first rock positions for any group in direction of tilt, plus final position as sentinel.
//
// col: [3, 10, 15, 20]
//
// Now, when adding a rock in that column (x, 0 <= y < 20), just find the insertion index in the ordered
// list and increment an entry for *that* location (x, insert_point(y)) in a map. This way each stationary
// rock is associated with the number of rolling rocks piled up against it.
//
// To enumerate the set of tilted rocks, it's easy to calculate what the N locations would be since they
// are just contiguous from the stationary rock (in a downward Y direction).
pub fn run(input: &str) -> (u32, u32) {
    let grid = AGrid::from_lines(input);

    let cycle = Cycle::new(&grid);

    let points: Vec<Point<u32>> = grid.all_points()
        .filter(|p| p.chr == 'O')
        // Switch to lower-left origin (since "North" is up in the data).
        .map(|p| Point::new(p.pos.x as u32, ((grid.height() - 1) - p.pos.y) as u32))
        .collect();
    let tilted_north = RockState::from(&cycle.maps[0], &points).get_tilted();
    (score(&tilted_north), get_billionth_score(&cycle, points))
}

fn score(points: &[Point<u32>]) -> u32 {
    points.iter().map(|p| p.y + 1).sum()
}

fn get_billionth_score(cycle: &Cycle, mut points: Vec<Point<u32>>) -> u32 {
    // Using the score to determine an exact match of cycles is dubious, but happens to work.
    // Really that's just the hash value and you should have the vector stored to test against.
    let mut cycles_state: HashMap<u32, u32> = HashMap::new();
    let mut cycle_count = 0u32;
    let mut end_cycle = None;
    loop {
        cycle_count += 1;
        points = cycle.spin(&points);
        if cycle_count > 50 && end_cycle.is_none() {
            if let Some(seen_before) = cycles_state.insert(score(&points), cycle_count) {
                let cycle_length = cycle_count - seen_before;
                end_cycle = Some(cycle_count + (1_000_000_000 - cycle_count) % cycle_length);
            }
        }
        if end_cycle.is_some_and(|end| cycle_count == end) {
            return score(&points);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum CardinalFrame {
    North(u32, u32),
    West(u32, u32),
    South(u32, u32),
    East(u32, u32),
}

impl CardinalFrame {
    pub fn to_local_frame(&self, global: &Point<u32>) -> Point<u32> {
        match &self {
            North(_, _) => *global,
            West(w, _) => Point::new(global.y, (*w - 1) - global.x),
            South(w, h) => Point::new((*w - 1) - global.x, (*h - 1) - global.y),
            East(_, h) => Point::new((*h - 1) - global.y, global.x),
        }
    }

    pub fn to_global_frame(&self, local: &Point<u32>) -> Point<u32> {
        match &self {
            North(_, _) => *local,
            West(_, h) => Point::new((*h - 1) - local.y, local.x),
            South(w, h) => Point::new((*w - 1) - local.x, (*h - 1) - local.y),
            East(w, _) => Point::new(local.y, (*w - 1) - local.x),
        }
    }

    pub fn width(&self) -> u32 {
        match &self {
            North(w, _) => *w,
            West(w, _) => *w,
            South(w, _) => *w,
            East(w, _) => *w,
        }
    }

    pub fn height(&self) -> u32 {
        match &self {
            North(_, h) => *h,
            West(_, h) => *h,
            South(_, h) => *h,
            East(_, h) => *h,
        }
    }
}

// Always tilt "forward" relative to local coordinates.
#[derive(Debug)]
struct TiltMap {
    cols: Vec<Vec<u32>>,
    orientation: CardinalFrame,
}

impl TiltMap {
    fn new(orientation: CardinalFrame, points: &[Point<u32>]) -> TiltMap {
        let mut cols: Vec<Vec<u32>> = vec![];
        for _ in 0..orientation.width() {
            cols.push(vec![orientation.height()]);
        }
        points.iter()
            .map(|p| orientation.to_local_frame(p))
            .for_each(|Point { x, y }| cols[x as usize].push(y));
        cols.iter_mut().for_each(|c| c.sort_unstable());
        TiltMap { cols, orientation }
    }

    fn get_stop(&self, p: &Point<u32>) -> u32 {
        let col = &self.cols[p.x as usize];
        let stop_idx = col.partition_point(|&v| v < p.y);
        col[stop_idx]
    }
}

#[derive(Debug)]
struct RockState<'a> {
    tilt_map: &'a TiltMap,
    rocks: HashMap<Point<u32>, u32>,
}

impl<'a> RockState<'a> {
    fn from(tilt_map: &'a TiltMap, points: &[Point<u32>]) -> RockState<'a> {
        let mut rocks: HashMap<Point<u32>, u32> = HashMap::new();

        points.iter().for_each(|p| assert!(p.x < tilt_map.orientation.width() && p.y < tilt_map.orientation.height()));

        for p in points.iter().map(|p| tilt_map.orientation.to_local_frame(&p)) {
            assert!(p.x < tilt_map.orientation.width() && p.y < tilt_map.orientation.height());
            *rocks.entry(Point { x: p.x, y: tilt_map.get_stop(&p) }).or_insert(0) += 1;
        }
        RockState { tilt_map, rocks }
    }

    fn get_tilted(&'a self) -> Vec<Point<u32>> {
        self.rocks.iter()
            .flat_map(|(p, &n)| (1..=n).map(|i| Point::new(p.x, p.y - i)))
            .map(|p| self.tilt_map.orientation.to_global_frame(&p))
            .collect()
    }
}

#[derive(Debug)]
struct Cycle {
    maps: [TiltMap; 4],
}

impl Cycle {
    fn new(grid: &AGrid) -> Cycle {
        let width = grid.width() as u32;
        let height = grid.height() as u32;
        let fixed: Vec<Point<u32>> = grid.all_points()
            .filter(|p| p.chr == '#')
            // Switch to lower-left origin (since "North" is up in the data).
            .map(|p| Point::new(p.pos.x as u32, (height - 1) - p.pos.y as u32))
            .collect();
        Cycle {
            maps: [
                TiltMap::new(North(width, height), &fixed),
                TiltMap::new(West(width, height), &fixed),
                TiltMap::new(South(width, height), &fixed),
                TiltMap::new(East(width, height), &fixed),
            ]
        }
    }

    fn rotate(&self, n: usize, points: &[Point<u32>]) -> Vec<Point<u32>> {
        RockState::from(&self.maps[n], points).get_tilted()
    }

    fn spin(&self, points: &[Point<u32>]) -> Vec<Point<u32>> {
        self.rotate(3, &self.rotate(2, &self.rotate(1, &self.rotate(0, points))))
    }
}
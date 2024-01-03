use std::collections::HashSet;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

use crate::agrid::{AGrid, GPoint};
use crate::xy::{Dir, Point};
use crate::xy::Dir::{Down, Left, Right, Up};

pub fn run(input: &str) -> (u32, u32) {
    let grid = AGrid::from_lines(input);

    let top_left = Point::new(0, 0);
    let bottom_right = Point::new(grid.width() - 1, grid.height() - 1);
    let mut starts: Vec<(Point<usize>, Dir)> = vec![];
    starts.extend(grid.points_after(&top_left, Down).map(|p| (p.pos, Right)));
    starts.extend(grid.points_after(&top_left, Right).map(|p| (p.pos, Down)));
    starts.extend(grid.points_after(&bottom_right, Left).map(|p| (p.pos, Up)));
    starts.extend(grid.points_after(&bottom_right, Up).map(|p| (p.pos, Left)));

    let results: Vec<u32> = starts.par_iter().map(|&(p, d)| Solver::solve(&grid, p, d)).collect();
    (results[0], *results.iter().max().unwrap())
}


struct Solver<'a> {
    grid: &'a AGrid,
    visited: HashSet<Point<usize>>,
}

impl<'a> Solver<'a> {
    fn solve(grid: &AGrid, start: Point<usize>, dir: Dir) -> u32 {
        let mut solver = Solver { grid, visited: HashSet::new() };
        if let Some(d) = solver.visit(&grid.at(&start), dir) {
            solver.follow_after(start, d);
        }
        solver.visited.len() as u32
    }

    fn follow_after(&mut self, mut pos: Point<usize>, mut dir: Dir) {
        // Assume start is already in visited.
        'outer: loop {
            for p in self.grid.points_after(&pos, dir) {
                if let Some(d) = self.visit(&p, dir) {
                    if d == dir {
                        continue;
                    }
                    dir = d;
                    pos = p.pos;
                    continue 'outer;
                } else {
                    break 'outer;
                }
            }
            break 'outer;
        }
    }

    fn visit(&mut self, p: &GPoint, dir: Dir) -> Option<Dir> {
        let unvisited = self.visited.insert(p.pos);
        if let (Right | Left, '|') = (dir, p.chr) {
            if unvisited {
                self.follow_after(p.pos, Up);
                self.follow_after(p.pos, Down);
            }
            return None;
        } else if let (Down | Up, '-') = (dir, p.chr) {
            if unvisited {
                self.follow_after(p.pos, Left);
                self.follow_after(p.pos, Right);
            }
            return None;
        }
        Some(match (dir, p.chr) {
            (Right, '\\') | (Left, '/') => Down,
            (Down, '\\') | (Up, '/') => Right,
            (Left, '\\') | (Right, '/') => Up,
            (Up, '\\') | (Down, '/') => Left,
            _ => dir,
        })
    }
}

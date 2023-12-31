use std::collections::HashMap;

use Dir::Left;

use crate::agrid::{AGrid, GPoint};
use crate::xy::{Dir, Point};
use crate::xy::Dir::{Down, Right, Up};

pub fn run(input: &str) -> (u32, u32) {
    let grid: AGrid = input.lines().collect();
    let points: Vec<GPoint> = grid.all_points()
        .filter(|p| is_part(p.chr))
        .collect();

    let mut map: HashMap<Point<usize>, Vec<u32>> = HashMap::new();
    let mut gears: Vec<u32> = vec![];
    for p in &points {
        let nums = grid.numbers_around(&p.pos);
        if p.chr == '*' && nums.len() == 2 {
            gears.push(nums[0] * nums[1]);
        }
        map.insert(p.pos, nums);
    }
    (map.values().flat_map(|v| v).sum::<u32>(), gears.iter().sum::<u32>())
}

fn is_part(c: char) -> bool {
    c != '.' && !c.is_ascii_digit()
}

impl AGrid {
    fn numbers_around(&self, p: &Point<usize>) -> Vec<u32> {
        let mut vec: Vec<u32> = vec![];
        for s in [p.move_by(1, Up), *p, p.move_by(1, Down)] {
            if !push_if(&self.number_at(&s), &mut vec) {
                push_if(&self.number_at(&s.move_by(1, Left)), &mut vec);
                push_if(&self.number_at(&s.move_by(1, Right)), &mut vec);
            }
        }
        vec
    }

    fn number_at(&self, p: &Point<usize>) -> Option<u32> {
        if let Some(lhs) =
            self.points_from(p, Left).take_while(|p| p.chr.is_ascii_digit()).last() {
            return self.points_from(&lhs.pos, Right)
                .take_while(|p| p.chr.is_ascii_digit())
                .map(|p| p.chr.to_digit(10).unwrap())
                .reduce(|m, n| (10 * m) + n);
        }
        return None;
    }
}

fn push_if(v: &Option<u32>, vec: &mut Vec<u32>) -> bool {
    if let Some(n) = v {
        vec.push(*n);
        true
    } else { false }
}

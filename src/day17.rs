use ndarray::Array2;
use pathfinding::directed::astar;

use Orientation::{Horizontal, Vertical};

use crate::xy::Orientation;

type Node = (u32, u32, Option<Orientation>);

pub fn run(input: &str) -> (u32, u32) {
    let lines: Vec<&str> = input.lines().collect();
    let width = lines[0].len();
    let height = lines.len();
    let digits: Vec<u8> = lines.iter().flat_map(|&s| s.chars().map(|c| c.to_digit(10).unwrap() as u8)).collect();
    let arr: Array2<u8> = Array2::from_shape_vec((height, width), digits).unwrap();

    let end_x = width as u32 - 1;
    let end_y = height as u32 - 1;

    let (_, part1) = astar::astar(
        &(0u32, 0u32, None),
        |n| successors(&arr, n, 1, 3),
        |n| n.0.abs_diff(end_x) + n.1.abs_diff(end_y),
        |n| n.0 == end_x && n.1 == end_y).unwrap();

    let (_, part2) = astar::astar(
        &(0u32, 0u32, None),
        |n| successors(&arr, n, 4, 10),
        |n| n.0.abs_diff(end_x) + n.1.abs_diff(end_y),
        |n| n.0 == end_x && n.1 == end_y).unwrap();

    (part1, part2)
}

fn successors(arr: &Array2<u8>, n: &Node, min: u32, max: u32) -> Vec<(Node, u32)> {
    let mut out: Vec<(Node, u32)> = Vec::with_capacity(2 * (max - min + 1) as usize);
    let &(x, y, hv) = n;
    match hv {
        Some(Vertical) => {
            let row = arr.row(y as usize);

            let mut right_cost: u32 = 0;
            for i in 1..=max {
                if let Some(n) = row.get((x + i) as usize) {
                    right_cost += *n as u32;
                    if i >= min {
                        out.push(((x + i, y, Some(Horizontal)), right_cost));
                    }
                } else { break; }
            }

            let mut left_cost: u32 = 0;
            for i in 1..=max {
                if let Some(n) = row.get((x - i) as usize) {
                    left_cost += *n as u32;
                    if i >= min {
                        out.push(((x - i, y, Some(Horizontal)), left_cost));
                    }
                } else { break; }
            }
            out
        }
        Some(Horizontal) => {
            let col = arr.column(x as usize);

            let mut down_cost: u32 = 0;
            for i in 1..=max {
                if let Some(n) = col.get((y + i) as usize) {
                    down_cost += *n as u32;
                    if i >= min {
                        out.push(((x, y + i, Some(Vertical)), down_cost));
                    }
                } else { break; }
            }

            let mut up_cost: u32 = 0;
            for i in 1..=max {
                if let Some(n) = col.get((y - i) as usize) {
                    up_cost += *n as u32;
                    if i >= min {
                        out.push(((x, y - i, Some(Vertical)), up_cost));
                    }
                } else { break; }
            }
            out
        }
        None => vec![((x, y, Some(Horizontal)), 0), ((x, y, Some(Vertical)), 0)],
    }
}

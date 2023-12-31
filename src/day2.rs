use std::collections::HashMap;

use lazy_static::lazy_static;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

#[derive(Display, EnumIter, Debug)]
enum RGB {
    RED = 0,
    GREEN = 1,
    BLUE = 2,
}

lazy_static! {
    // Map names and string values to integers.
    static ref RGB_MAP: HashMap<String, usize> = HashMap::from_iter(
        RGB::iter().map(|c| (c.to_string().to_ascii_lowercase(), c as usize)));
}

pub fn run(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let mut part1 = 0;
    let mut part2 = 0;
    for game in lines {
        let m: &mut [i32] = &mut [0, 0, 0];
        let (lhs, rhs) = game.split_once(':').unwrap();
        let id: i32 = lhs.split_once(' ').unwrap().1.parse().unwrap();
        rhs.split(';')
            .flat_map(|h| h.split(',').map(str::trim))
            .map(|c| c.split_once(' ').unwrap())
            .map(|(n, rgb)| (n.parse::<i32>().unwrap(), *RGB_MAP.get(rgb).unwrap()))
            .for_each(|(n, c)| if n > m[c] { m[c] = n });
        if let [red, green, blue] = m {
            // 12 red cubes, 13 green cubes, and 14 blue cubes
            if *red <= 12 && *green <= 13 && *blue <= 14 { part1 += id; }
            part2 += *red * *green * *blue;
        }
    }
    (part1, part2)
}


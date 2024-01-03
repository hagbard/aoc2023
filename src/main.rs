extern crate core;

use std::fmt::Debug;
use std::fs::read_to_string;
use std::time::Instant;

mod util;
mod rpoly;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day7;
mod day8;
mod day10;
mod day18;
mod agrid;
mod xy;
mod day9;
mod day11;
mod day12_alt;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    time(1, &day1::run);
    time(2, &day2::run);
    time(3, &day3::run);
    time(4, &day4::run);
    time(5, &day5::run);
    time(7, &day7::run);
    time(8, &day8::run);
    time(9, &day9::run);
    time(10, &day10::run);
    time(11, &day11::run);
    time(12, &day12_alt::run);
    time(13, &day13::run);
    time(14, &day14::run);
    time(15, &day15::run);
    time(16, &day16::run);
    time(18, &day18::run);
}

fn time<I: Debug>(num: u32, fnc: &dyn Fn(&str) -> (I, I)) {
    // panic on possible file-reading errors
    let input =
        read_to_string(&format!("day{}.txt", num)).unwrap().replace("\r\n", "\n");
    let start = Instant::now();
    let (p1, p2) = fnc(&input);
    let taken = start.elapsed();
    println!("Day {} [{} ms] (1) = {:?}, (2) = {:?}", num, taken.as_millis(), p1, p2);
}

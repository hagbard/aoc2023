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
mod day17;

fn main() {
    let mut total_micros: u128 = 0;
    total_micros += time(1, &day1::run);
    total_micros += time(2, &day2::run);
    total_micros += time(3, &day3::run);
    total_micros += time(4, &day4::run);
    total_micros += time(5, &day5::run);
    total_micros += time(7, &day7::run);
    total_micros += time(8, &day8::run);
    total_micros += time(9, &day9::run);
    total_micros += time(10, &day10::run);
    total_micros += time(11, &day11::run);
    total_micros += time(12, &day12_alt::run);
    total_micros += time(13, &day13::run);
    total_micros += time(14, &day14::run);
    total_micros += time(15, &day15::run);
    total_micros += time(16, &day16::run);
    total_micros += time(17, &day17::run);
    total_micros += time(18, &day18::run);
    println!("Total time: {} µs", total_micros);
}

fn time<I: Debug>(num: u32, fnc: &dyn Fn(&str) -> (I, I)) -> u128 {
    // panic on possible file-reading errors
    let input =
        read_to_string(&format!("day{}.txt", num)).unwrap().replace("\r\n", "\n");
    let start = Instant::now();
    let (p1, p2) = fnc(&input);
    let taken = start.elapsed();
    let taken_micros = taken.as_micros();
    println!("Day {} [{} µs] (1) = {:?}, (2) = {:?}", num, taken_micros, p1, p2);
    taken_micros
}

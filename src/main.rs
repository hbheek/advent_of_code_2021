use std::env;

mod days;

use days::*;

fn main() {
    let day: u8 = env::args().nth(1).unwrap().parse().unwrap();
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        4 => day4::run(),
        5 => day5::run(),
        6 => day6::run(),
        7 => day7::run(),
        _ => println!("A little patience, please!"),
    }
}

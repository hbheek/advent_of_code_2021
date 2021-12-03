use std::env;

mod days;

use days::{day1, day2, day3};

fn main() {
    let day: u8 = env::args().nth(1).unwrap().parse().unwrap();
    match day {
        1 => day1::run(),
        2 => day2::run(),
        3 => day3::run(),
        _ => println!("A little patience, please!"),
    }
}

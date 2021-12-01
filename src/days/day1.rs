use std::{fs::File, io::{BufReader, BufRead}};

pub fn run() {
    let file = File::open("input/day1.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let nums: Vec<i16> = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.parse::<i16>().ok())
        .collect();

    let res1 = nums
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();
    println!("{}", res1);

    let sums: Vec<i16> = nums
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect();
    let res2 = sums
        .windows(2)
        .filter(|w| w[1] > w[0])
        .count();
    println!("{}", res2);
}

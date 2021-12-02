use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let file = File::open("input/day2.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let commands: Vec<(String, usize)> = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .map(parse_line)
        .collect();
    
    // println!("{:?}", commands.len());

    let pos = commands
        .iter()
        .fold((0, 0), |(hor, depth), (movement, amount)| {
            match &movement[..] {
                "forward" => (hor + amount, depth),
                "down" => (hor, depth + amount),
                "up" => (hor, depth - amount),
                _ => panic!("invalid command"),
            }
        });
    let res1 = pos.0 * pos.1;
    println!("{}", res1);

    let state = commands
        .iter()
        .fold((0, 0, 0), |(hor, depth, aim), (movement, amount)| {
            match &movement[..] {
                "up" => (hor, depth, aim - amount),
                "down" => (hor, depth, aim + amount),
                "forward" => (hor + amount, depth + amount * aim, aim),
                _ => panic!("invalid command"),
            }
        });
    let res2 = state.0 * state.1;
    println!("{}", res2);
}

fn parse_line(line: String) -> (String, usize) {
    let split: Vec<&str> = line.split(' ').collect();
    let movement = split[0].to_string();
    let amount: usize = split[1].parse().unwrap();
    (movement, amount)
}

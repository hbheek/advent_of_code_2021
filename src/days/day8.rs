use std::{
    fs::File,
    io::{BufRead, BufReader},
};

type Input = Vec<(Vec<String>, Vec<String>)>;

pub fn run() {
    let file = File::open("input/day8.txt").unwrap();
    let input: Input = BufReader::new(file)
        .lines()
        .flat_map(|line| line.ok())
        .flat_map(parse_line)
        .collect();

    let res1: usize = input
        .iter()
        .map(|(_signals, outputs)| counts_outputs(outputs))
        .sum();
    println!("{}", res1);
}

fn counts_outputs(outputs: &Vec<String>) -> usize {
    let mut count = 0;
    for output in outputs {
        let n = output.len();
        if n == 2 || n == 4 || n == 3 || n == 7 {
            count += 1;
        }
    }
    count
}

fn parse_line(line: String) -> Option<(Vec<String>, Vec<String>)> {
    if let Some((signals, outputs)) = line.split_once('|') {
        let signals: Vec<String> = signals
            .split_whitespace()
            .map(|signal| signal.to_string())
            .collect();
        let outputs: Vec<String> = outputs
            .split_whitespace()
            .map(|output| output.to_string())
            .collect();
        Some((signals, outputs))
    } else {
        None
    }
}

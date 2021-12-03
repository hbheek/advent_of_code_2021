use std::{
    cmp::Ordering::{Equal, Greater, Less},
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let file = File::open("input/day3.txt").unwrap();
    let buf_reader = BufReader::new(file);
    let input: Vec<Vec<u8>> = buf_reader
        .lines()
        .filter_map(|line| line.ok())
        .map(parse_line)
        .collect();

    // part 1

    let bit_counters = input
        .iter()
        .map(get_bit_counter)
        .reduce(add_bit_counters)
        .unwrap();
    let gamma = get_gamma(&bit_counters);
    let epsilon = get_epsilon(&bit_counters);
    println!("{}", gamma * epsilon);

    // part 2

    let mut copy1 = input.clone();
    let mut pos = 0;
    while copy1.len() > 1 {
        let msb = get_msb(&copy1, pos).unwrap_or(1);
        copy1.retain(|bits| bits[pos] == msb);
        pos += 1;
    }
    let oxygen = bits_to_int(&copy1[0]);

    let mut copy2 = input.clone();
    let mut pos = 0;
    while copy2.len() > 1 {
        let msb = get_msb(&copy2, pos).unwrap_or(1);
        let lsb = 1 - msb;
        copy2.retain(|bits| bits[pos] == lsb);
        pos += 1;
    }
    let co2 = bits_to_int(&copy2[0]);

    println!("{}", oxygen * co2);
}

fn parse_line(line: String) -> Vec<u8> {
    line.chars()
        .filter_map(|char| match char {
            '0' => Some(0),
            '1' => Some(1),
            _ => None,
        })
        .collect()
}

fn get_bit_counter(bits: &Vec<u8>) -> Vec<(usize, usize)> {
    bits.iter()
        .map(|&bit| if bit == 0 { (1, 0) } else { (0, 1) })
        .collect()
}

fn add_bit_counters(c1: Vec<(usize, usize)>, c2: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    c1.iter()
        .zip(c2.iter())
        .map(|(a, b)| (a.0 + b.0, a.1 + b.1))
        .collect()
}

fn bits_to_int(bits: &Vec<u8>) -> u32 {
    let bits: String = bits
        .iter()
        .map(|&bit| if bit == 0 { '0' } else { '1' })
        .collect();
    u32::from_str_radix(&bits, 2).unwrap()
}

fn get_gamma(bit_counters: &Vec<(usize, usize)>) -> u32 {
    let msbs: Vec<u8> = bit_counters
        .iter()
        .map(|(num0s, num1s)| if num0s > num1s { 0 } else { 1 })
        .collect();
    bits_to_int(&msbs)
}

fn get_epsilon(bit_counters: &Vec<(usize, usize)>) -> u32 {
    let lsbs: Vec<u8> = bit_counters
        .iter()
        .map(|(num0s, num1s)| if num0s < num1s { 0 } else { 1 })
        .collect();
    bits_to_int(&lsbs)
}

fn get_msb(input: &Vec<Vec<u8>>, pos: usize) -> Option<u8> {
    let num_bits = input.len();
    let num0s = input.iter().filter(|bits| bits[pos] == 0).count();
    let num1s = num_bits - num0s;
    match num0s.cmp(&num1s) {
        Less => Some(1),
        Equal => None,
        Greater => Some(0),
    }
}

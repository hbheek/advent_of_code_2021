use std::fs::read_to_string;

pub fn run() {
    let mut crabs: Vec<usize> = read_to_string("input/day7.txt")
        .unwrap()
        .split(',')
        .filter_map(|t| t.parse::<usize>().ok())
        .collect();

    let median = get_median(&mut crabs);
    let min_fuel_p1 = calc_fuel_p1(&crabs, median);
    println!("{}", min_fuel_p1);

    let mut unique_xs = crabs.clone();
    unique_xs.dedup();
    let min_fuel_p2 = unique_xs
        .iter()
        .map(|&x| calc_fuel_p2(&crabs, x))
        .min()
        .unwrap();
    println!("{}", min_fuel_p2);
}

fn get_median(crabs: &mut Vec<usize>) -> usize {
    crabs.sort();
    let n = crabs.len();
    (crabs[n / 2 - 1] + crabs[n / 2]) / 2
}

fn calc_fuel_p1(crabs: &Vec<usize>, pos: usize) -> usize {
    let mut fuel = 0;
    for &crab_pos in crabs {
        fuel += usize::max(pos, crab_pos) - usize::min(crab_pos, pos);
    }
    fuel
}

fn calc_fuel_p2(crabs: &Vec<usize>, pos: usize) -> usize {
    let mut fuel = 0;
    for &crab_pos in crabs {
        let d = usize::max(pos, crab_pos) - usize::min(crab_pos, pos);
        fuel += d * (d + 1) / 2;
    }
    fuel
}

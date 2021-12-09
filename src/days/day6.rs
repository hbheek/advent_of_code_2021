use std::fs::read_to_string;

pub fn run() {
    let mut fish = [0; 9];
    read_to_string("input/day6.txt")
        .unwrap()
        .split(',')
        .filter_map(|t| t.parse::<usize>().ok())
        .for_each(|t| fish[t] += 1);

    let res1 = solve(fish, 80);
    println!("{}", res1);

    let res2 = solve(fish, 256);
    println!("{}", res2);
}

fn solve(init_fish: [usize; 9], days: usize) -> usize {
    let mut fish = init_fish;
    for _ in 0..days {
        let mut temp_fish = [0; 9];
        for t in 0..=8 {
            if t == 0 {
                temp_fish[6] += fish[t];
                temp_fish[8] += fish[t];
            } else {
                temp_fish[t - 1] += fish[t];
            }
        }
        fish = temp_fish;
    }
    fish.iter().sum()
}

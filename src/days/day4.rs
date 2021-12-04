use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub fn run() {
    let file = File::open("input/day4.txt").unwrap();
    let mut lines = BufReader::new(file).lines().flat_map(|line| line.ok());

    let nums: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .flat_map(|num| num.parse().ok())
        .collect();

    let mut boards = Vec::new();
    while lines.next().is_some() {
        boards.push(Board::from_lines(&mut lines));
    }

    part1(nums.clone(), &mut boards.clone());
    part2(nums.clone(), &mut boards.clone());
}

fn part1(nums: Vec<u32>, boards: &mut Vec<Board>) {
    let mut res1 = 0;
    'outer: for num in nums {
        for board in boards.iter_mut() {
            board.cross(num);
            if board.won() {
                res1 = board.score(num);
                break 'outer;
            }
        }
    }
    println!("{}", res1);
}

fn part2(nums: Vec<u32>, boards: &mut Vec<Board>) {
    let mut num = 0;
    let mut nums = nums.into_iter();
    while boards.len() > 1 {
        num = nums.next().unwrap();
        boards.iter_mut().for_each(|board| board.cross(num));
        boards.retain(|board| !board.won());
    }
    let last_board = &mut boards[0];
    while !last_board.won() {
        num = nums.next().unwrap();
        last_board.cross(num);
    }
    let res2 = last_board.score(num);
    println!("{}", res2);
}

#[derive(Clone)]
struct Board(Vec<Vec<(u32, bool)>>);

impl Board {
    fn from_lines(lines: &mut impl Iterator<Item = String>) -> Self {
        let mut grid = Vec::new();
        for _ in 0..5 {
            let line = lines.next().unwrap();
            grid.push(
                line.split_ascii_whitespace()
                    .map(|num| (num.parse().unwrap(), false))
                    .collect(),
            );
        }
        Board(grid)
    }

    fn cross(&mut self, num: u32) {
        for row in 0..5 {
            for col in 0..5 {
                if self.0[row][col].0 == num {
                    self.0[row][col].1 = true;
                }
            }
        }
    }

    fn won(&self) -> bool {
        let mut won = false;
        // check rows
        for row in 0..5 {
            let mut row_won = true;
            for col in 0..5 {
                row_won = row_won && self.0[row][col].1;
            }
            won = won || row_won;
        }
        // check cols
        for col in 0..5 {
            let mut col_won = true;
            for row in 0..5 {
                col_won = col_won && self.0[row][col].1;
            }
            won = won || col_won;
        }
        won
    }

    fn score(&self, num: u32) -> u32 {
        let mut unmarked_sum = 0;
        for row in 0..5 {
            for col in 0..5 {
                if !self.0[row][col].1 {
                    unmarked_sum += self.0[row][col].0;
                }
            }
        }
        unmarked_sum * num
    }
}

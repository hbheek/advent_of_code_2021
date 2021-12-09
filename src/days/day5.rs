use std::{
    collections::BTreeMap,
    fs::File,
    io::{BufRead, BufReader},
};

use regex::Regex;

pub fn run() {
    let file = File::open("input/day5.txt").unwrap();
    let lines: Vec<Line> = BufReader::new(file)
        .lines()
        .flat_map(|line| line.ok())
        .map(|line| Line::from_str(&line))
        .collect();

    let mut cover_points = BTreeMap::new();
    for line in &lines {
        if line.is_straight() {
            for point in line.get_points() {
                *cover_points.entry(point).or_insert(0) += 1;
            }
        }
    }
    let res1 = cover_points.iter().filter(|(_k, &v)| v >= 2).count();
    println!("{}", res1);

    for line in &lines {
        if !line.is_straight() {
            for point in line.get_points() {
                *cover_points.entry(point).or_insert(0) += 1;
            }
        }
    }
    let res2 = cover_points.iter().filter(|(_k, &v)| v >= 2).count();
    println!("{}", res2);
}

struct Line {
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
}

impl Line {
    fn from_str(s: &str) -> Self {
        let re = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
        let cap = re.captures(s).unwrap();
        let coords: Vec<usize> = cap
            .iter()
            .skip(1)
            .take(4)
            .map(|m| m.unwrap().as_str().parse().unwrap())
            .collect();
        Line {
            x0: coords[0],
            y0: coords[1],
            x1: coords[2],
            y1: coords[3],
        }
    }

    fn get_xs(&self) -> Vec<usize> {
        if self.x0 > self.x1 {
            (self.x1..=self.x0).rev().collect()
        } else {
            (self.x0..=self.x1).collect()
        }
    }

    fn get_ys(&self) -> Vec<usize> {
        if self.y0 > self.y1 {
            (self.y1..=self.y0).rev().collect()
        } else {
            (self.y0..=self.y1).collect()
        }
    }

    fn get_points(&self) -> Vec<(usize, usize)> {
        if self.x0 == self.x1 {
            self.get_ys().into_iter().map(|y| (self.x0, y)).collect()
        } else if self.y0 == self.y1 {
            self.get_xs().into_iter().map(|x| (x, self.y0)).collect()
        } else {
            self.get_xs()
                .into_iter()
                .zip(self.get_ys().into_iter())
                .collect()
        }
    }

    fn is_straight(&self) -> bool {
        self.x0 == self.x1 || self.y0 == self.y1
    }
}

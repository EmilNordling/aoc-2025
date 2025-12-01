use std::fs::read_to_string;
use aoc::year2025::day01::{parse, part1};

fn main() {
    let day = 1;
    let path = format!("input/year2025/day{day:02}.txt");

    if let Ok(data) = read_to_string(&path) {
        let input = parse(&data);
        println!("{}", part1(&input));
    } else {
        println!("Failed to read input file");
    }
}

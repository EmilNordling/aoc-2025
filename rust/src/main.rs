use aoc::year2025::day06::{parse, part1, part2};
use std::{fs::read_to_string, time::Instant};

fn main() {
    let day = 6;
    let path = format!("../input/year2025/day{day:02}.txt");

    if let Ok(data) = read_to_string(&path) {
        let input = parse(&data);

        let instant1 = Instant::now();
        let out_part1 = part1(&input);
        let elapsed1 = instant1.elapsed();
        println!("{out_part1} | time: {elapsed1:?}");

        let instant2 = Instant::now();
        let out_part2 = part2(&input);
        let elapsed2 = instant2.elapsed();
        println!("{out_part2} | time: {elapsed2:?}");
    } else {
        println!("Failed to read input file");
    }
}

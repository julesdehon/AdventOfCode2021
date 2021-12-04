use std::fs;
use std::str::{FromStr, Split};

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the input file");
    let lines = contents.split("\n");
    let lines2 = contents.split("\n");

    let increased_times = part1(lines);
    let increased_sliding_window_times = part2(lines2);

    println!("Increased {} times", increased_times);
    println!("Sliding window increased {} times", increased_sliding_window_times);
}

fn part1(lines: Split<&str>) -> i32 {
    let mut increased_times = 0;
    let mut prev = i32::MAX;
    for line in lines {
        let curr : i32 = FromStr::from_str(line).unwrap();
        if curr > prev {
            increased_times += 1;
        }
        prev = curr;
    }
    return increased_times;
}

fn part2(lines: Split<&str>) -> i32 {
    let numbered_lines: Vec<i32> = lines.map(|x| FromStr::from_str(x).unwrap()).collect();
    let mut increased_times = 0;
    let mut prev = numbered_lines[0] + numbered_lines[1] + numbered_lines[2];
    for i in 3..numbered_lines.len() {
        let curr : i32 = prev - numbered_lines[i - 3] + numbered_lines[i];
        if curr > prev {
            increased_times += 1;
        }
        prev = curr;
    }
    return increased_times;
}
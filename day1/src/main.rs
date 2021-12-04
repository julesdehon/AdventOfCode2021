use std::fs;
use std::str::FromStr;

fn main() {
    let contents = fs::read_to_string("src/input.txt")
        .expect("Something went wrong reading the input file");
    let lines = contents.split("\n");

    let mut increased_times = 0;
    let mut prev = i32::MAX;
    for line in lines {
        let curr : i32 = FromStr::from_str(line).unwrap();
        if curr > prev {
            increased_times += 1;
        }
        prev = curr;
    }

    println!("Increased {} times", increased_times);
}

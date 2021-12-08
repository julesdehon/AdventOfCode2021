use std::fs;
use std::str::FromStr;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32)
}

impl Command {
    fn parse(raw: &str) -> Result<Command, &'static str> {
        let split: Vec<&str> = raw.split(' ').collect();
        match split[0] {
            "forward" => Ok(Command::Forward(FromStr::from_str(split[1]).unwrap())),
            "down" => Ok(Command::Down(FromStr::from_str(split[1]).unwrap())),
            "up" => Ok(Command::Up(FromStr::from_str(split[1]).unwrap())),
            _ => Err("Could not match command")
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines = contents.split('\n');
    let commands: Vec<Command> = lines.map(|line| Command::parse(line).unwrap()).collect();

    let (pos, depth) = part1(&commands);
    println!("Reached position {}, and depth {} - multiplying gives {}", pos, depth, pos * depth);

    println!("Using new interpretation of commands...");
    let (pos, depth) = part2(&commands);
    println!("Reached position {}, and depth {} - multiplying gives {}", pos, depth, pos * depth);
}

fn part1(commands: &[Command]) -> (i32, i32) {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    for command in commands {
        match command {
            Command::Forward(i) => { pos += i }
            Command::Down(i) => { depth += i }
            Command::Up(i) => { depth -= i }
        }
    }
    (pos, depth)
}

fn part2(commands: &[Command]) -> (i32, i32) {
    let mut pos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for command in commands {
        match command {
            Command::Forward(i) => { pos += i; depth += aim * i }
            Command::Down(i) => { aim += i }
            Command::Up(i) => { aim -= i }
        }
    }
    (pos, depth)
}
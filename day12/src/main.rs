extern crate itertools;

use std::fs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");

    let mut caves = HashMap::new();
    contents.split('\n')
        .map(|line| line.split('-').next_tuple().unwrap())
        .for_each(|(first, second)| {
            let first_cave = caves.entry(first).or_insert(vec![]);
            first_cave.push(second);
            let second_cave = caves.entry(second).or_insert(vec![]);
            second_cave.push(first);
        });

    let num_paths = part1(&caves);
    println!("There are {} paths", num_paths);

    let num_paths_with_double_visit = part2(&caves);
    println!("When we are allowed to visit a cave twice, there are {} paths", num_paths_with_double_visit);
}

fn part1(caves: &HashMap<&str, Vec<&str>>) -> u32 {
    find_num_paths_to_end("start", &caves, HashSet::new(), true)
}

fn part2(caves: &HashMap<&str, Vec<&str>>) -> u32 {
    find_num_paths_to_end("start", &caves, HashSet::new(), false)
}

fn small(cave: &str) -> bool {
    cave.to_lowercase() == cave
}

fn find_num_paths_to_end(cave: &str, caves: &HashMap<&str, Vec<&str>>, mut visited: HashSet<String>, mut small_visited_twice: bool) -> u32 {
    if cave == "end" { return 1 }
    if small(cave) && visited.contains(cave) {
        if small_visited_twice || cave == "start" {
            return 0
        } else {
            small_visited_twice = true;
        }
    }
    visited.insert(cave.to_owned());
    caves.get(cave).unwrap().iter()
        .map(|neighbour| {
            find_num_paths_to_end(neighbour, caves,visited.clone(), small_visited_twice)
        })
        .sum()
}
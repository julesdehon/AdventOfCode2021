extern crate itertools;

use std::fs;
use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let (template, raw_rules) = contents.split_once("\n\n").unwrap();
    let rules = raw_rules.split('\n')
        .map(|line| line.split_once(" -> ").unwrap())
        .map(|(from, to)| (from.chars().next_tuple::<(char, char)>().unwrap(), to.chars().next().unwrap()))
        .collect();
    let diff_10_steps = diff(template, &rules, 10);
    let diff_40_steps = diff(template, &rules, 40);
    println!("If you take the quantity of the most common element and subtract the quantity of the least common element:");
    println!("After 10 steps you get {}, and after 40 you get {}", diff_10_steps, diff_40_steps);
}

fn diff(template: &str, rules: &HashMap<(char, char), char>, steps: u32) -> u64 {
    let mut pair_counts: HashMap<(char, char), u64> = HashMap::new();
    for i in 0..template.len() - 1 {
        *pair_counts.entry((template.chars().nth(i).unwrap(),
                            template.chars().nth(i + 1).unwrap())).or_insert(0) += 1;
    }
    for _ in 0..steps {
        for (p@(f, s), n) in pair_counts.clone().iter().filter(|(_, n)| **n > 0) {
            if let Some(c) = rules.get(p) {
                *pair_counts.get_mut(&(*f, *s)).unwrap() -= n;
                *pair_counts.entry((*f, *c)).or_insert(0) += n;
                *pair_counts.entry((*c, *s)).or_insert(0) += n;
            }
        }
    }
    // Doubled because every character is counted twice - once for the left pair,
    // and once for the right pair
    let mut char_count_doubled = pair_counts.iter().fold(HashMap::new(), |mut map, ((f, s), n)| {
        *map.entry(*f).or_insert(0) += n;
        *map.entry(*s).or_insert(0) += n;
        map
    });
    // Add 1 to the char count of the left-most and right-most characters, since they were
    // only counted once.
    *char_count_doubled.get_mut(&template.chars().next().unwrap()).unwrap() += 1;
    *char_count_doubled.get_mut(&template.chars().nth(template.len() - 1).unwrap()).unwrap() += 1;
    (char_count_doubled.values().max().unwrap() - char_count_doubled.values().min().unwrap()) / 2
}
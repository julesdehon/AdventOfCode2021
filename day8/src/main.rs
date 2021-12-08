extern crate itertools;

use std::fs;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

struct Decoder {
    mapping: HashMap<char, char>,
}

impl Decoder {
    fn new(inputs: &[&str]) -> Decoder {
        fn get_letter_counts(inputs: &[&str]) -> HashMap<u32, Vec<char>> {
            let chars: Vec<char> = inputs.iter().map(|input| input.chars().collect()).concat();
            let mut letter_counts = HashMap::new();
            for char in chars {
                let char_count = letter_counts.entry(char).or_insert(0);
                *char_count += 1;
            }
            let mut result = HashMap::new();
            for (letter, count) in letter_counts.iter() {
                let letters_with_count = result.entry(*count).or_insert_with(Vec::new);
                letters_with_count.push(*letter);
            }
            result
        }

        let mut mapping = HashMap::new();
        let four = inputs.iter().find(|input| input.len() == 4).unwrap();
        let letters_with_count = get_letter_counts(inputs);
        mapping.insert(letters_with_count.get(&6).unwrap()[0], 'b');
        mapping.insert(letters_with_count.get(&4).unwrap()[0], 'e');
        mapping.insert(letters_with_count.get(&9).unwrap()[0], 'f');

        // d is present in the digit 4
        let letters_appearing_7_times = letters_with_count.get(&7).unwrap();
        if four.contains(letters_appearing_7_times[0]) {
            mapping.insert(letters_appearing_7_times[0], 'd');
            mapping.insert(letters_appearing_7_times[1], 'g');
        } else {
            mapping.insert(letters_appearing_7_times[0], 'g');
            mapping.insert(letters_appearing_7_times[1], 'd');
        }

        // c is present in the digit 4
        let letters_appearing_8_times = letters_with_count.get(&8).unwrap();
        if four.contains(letters_appearing_8_times[0]) {
            mapping.insert(letters_appearing_8_times[0], 'c');
            mapping.insert(letters_appearing_8_times[1], 'a');
        } else {
            mapping.insert(letters_appearing_8_times[0], 'a');
            mapping.insert(letters_appearing_8_times[1], 'c');
        }

        Decoder {
            mapping,
        }
    }

    fn decode(&self, digit: &str) -> u32 {
        let digit_from_segments: Vec<(HashSet<char>, u32)> = vec![
            (HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']), 0),
            (HashSet::from(['c', 'f']), 1),
            (HashSet::from(['a', 'c', 'd', 'e', 'g']), 2),
            (HashSet::from(['a', 'c', 'd', 'f', 'g']), 3),
            (HashSet::from(['b', 'c', 'd', 'f',]), 4),
            (HashSet::from(['a', 'b', 'd', 'f', 'g']), 5),
            (HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']), 6),
            (HashSet::from(['a', 'c', 'f']), 7),
            (HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']), 8),
            (HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']), 9),
        ];

        let signals: HashSet<char> = digit.chars().map(|letter| *self.mapping.get(&letter).unwrap()).collect();
        digit_from_segments.iter().find(|(set, _)| *set == signals).unwrap().1
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines: Vec<&str> = contents.split('\n').collect();
    let num_unique_digits = part1(&lines);
    println!("The digits 1, 4, 7, or 8 appeared {} times in the output", num_unique_digits);
    let sum_outputs = part2(&lines);
    println!("The sum of all the output numbers was {}", sum_outputs);
}

fn part1(lines: &[&str]) -> u32 {
    let mut result = 0;
    for line in lines {
        let output = line.split(" | ").nth(1).unwrap();
        result += output.split(' ').fold(0, |accum, word| accum + match word.len() {
            2 => 1,
            4 => 1,
            3 => 1,
            7 => 1,
            _ => 0
        })
    }
    result
}

fn part2(lines: &[&str]) -> u32 {
    let mut result = 0;
    for line in lines {
        let (input, output): (&str, &str) = line.split(" | ").next_tuple().unwrap();
        let decoder = Decoder::new(&*input.split(' ').collect::<Vec<&str>>());
        let decoded_digits: Vec<u32> = output.split(' ').map(|signals| decoder.decode(signals)).collect();
        result += decoded_digits[0] * 1000 + decoded_digits[1] * 100 + decoded_digits[2] * 10 + decoded_digits[3];
    }
    result
}
use std::fs;

fn syntax_checker_score(illegal_char: char) -> u32 {
    match illegal_char {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("Unknown illegal character!")
    }
}

fn autocomplete_score(illegal_char: char) -> u64 {
    match illegal_char {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("Unknown illegal character!")
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines: Vec<Vec<char>> = contents.split('\n').map(|line| line.chars().collect()).collect();

    let (syntax_error_score, autocomplete_score) = part1(&lines);
    println!("The total syntax error score was {}", syntax_error_score);
    println!("The total autocomplete score was {}", autocomplete_score);
}

fn part1(lines: &[Vec<char>]) -> (u32, u64) {
    let mut syntax_checker_result = 0;
    let mut autocomplete_results = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut contains_error = false;
        for symbol in line {
            match symbol {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),
                ')' | ']' | '}' | '>' => if stack.pop().unwrap() != *symbol { syntax_checker_result += syntax_checker_score(*symbol); contains_error = true; },
                _ => panic!("Unknown symbol"),
            }
        }
        if contains_error { continue }
        autocomplete_results.push(stack.iter().rev().fold(0, |score, char| score * 5 + autocomplete_score(*char)));
    }
    autocomplete_results.sort_unstable();
    (syntax_checker_result, autocomplete_results[autocomplete_results.len() / 2])
}

use std::fs;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone)]
struct Board {
    number_rows: Vec<Vec<u32>>,
    marked: Vec<(usize, usize)>,
    row_counts: HashMap<usize, u32>,
    column_counts: HashMap<usize, u32>,
    bingo: bool,
}

impl Board {
    fn new(number_rows: Vec<Vec<u32>>) -> Board {
        return Board {
            number_rows,
            marked: vec![],
            row_counts: HashMap::new(),
            column_counts: HashMap::new(),
            bingo: false
        }
    }

    fn parse(raw: &str) -> Board {
        let mut number_rows = vec![];
        let rows = raw.split('\n');
        for (i, row) in rows.enumerate() {
            number_rows.push(vec![]);
            let nums = row.split_whitespace();
            for num_str in nums {
                let num: u32 = FromStr::from_str(num_str).unwrap();
                number_rows[i].push(num);
            }
        }
        return Board::new(number_rows);
    }

    fn mark(&mut self, num: u32) -> bool {
        for row_idx in 0..self.number_rows.len() {
            for col_idx in 0..self.number_rows.len() {
                if self.number_rows[row_idx][col_idx] == num {
                    self.marked.push((row_idx, col_idx));

                    let row_count = self.row_counts.entry(row_idx).or_insert(0);
                    *row_count += 1;
                    if *row_count >= self.number_rows.len() as u32 {
                        self.bingo = true;
                    }

                    let col_count = self.column_counts.entry(col_idx).or_insert(0);
                    *col_count += 1;
                    if *col_count >= self.number_rows.len() as u32 {
                        self.bingo = true;
                    }
                }
            }
        }
        return self.bingo;
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines: Vec<&str> = contents.split("\n").collect();

    let draws: Vec<u32> = lines[0].split(',').map(|num_str| FromStr::from_str(num_str).unwrap()).collect();
    let part1_boards: Vec<Board> = lines[2..].split(|line| line.is_empty())
        .map(|raw_board_lines| {
            let raw_board = raw_board_lines.join("\n");
            return Board::parse(&raw_board)
        }).collect();

    let part2_boards = part1_boards.clone();

    let score = part1(&draws, part1_boards).expect("There was no winning board");
    println!("Final score was {}", score);

    let losing_score = part2(&draws, part2_boards).expect("There was no winning board");
    println!("Final score of losing board was {}", losing_score);
}

fn part2(draws: &Vec<u32>, boards: Vec<Board>) -> Option<u32> {
    let mut still_playing_boards = boards;
    for draw in draws {
        let mut next_still_playing_boards = vec![];
        let num_boards = still_playing_boards.len();
        for mut board in still_playing_boards {
            if num_boards == 1 && board.mark(*draw) {
                return Some(draw * sum_unmarked_numbers(&board));
            }
            if !board.mark(*draw) {
                next_still_playing_boards.push(board);
            }
        }
        still_playing_boards = next_still_playing_boards;
    }
    return None;
}

fn part1(draws: &Vec<u32>, mut boards: Vec<Board>) -> Option<u32> {
    for draw in draws {
        for board in boards.iter_mut() {
            if board.mark(*draw) {
                return Some(draw * sum_unmarked_numbers(board));
            }
        }
    }
    return None;
}

fn sum_unmarked_numbers(board: &Board) -> u32 {
    let total_sum = board.number_rows.iter().fold(0, |accum, row| accum + row.iter().fold(0, |accum, num| accum + num));
    let marked_sum = board.marked.iter().fold(0, |accum, (row_idx, col_idx)| board.number_rows[*row_idx][*col_idx] + accum);
    return total_sum - marked_sum;
}
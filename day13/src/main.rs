use std::fs;
use std::fmt;

enum Fold {
    Up,
    Left,
}

struct Paper {
    dots: Vec<Vec<bool>>,
}

impl fmt::Debug for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in &self.dots {
            line.iter().for_each(|space| write!(f, "{}", match space {
                true => '#',
                false => '.',
            }).unwrap());
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Paper {
    fn parse(raw: &str) -> Paper {
        let points = raw.split('\n')
            .map(|line| line.split_once(',').unwrap())
            .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()));
        let (max_x, max_y) = points.clone()
            .fold((0, 0), |(max_x, max_y), (next_x, next_y)| (max_x.max(next_x), max_y.max(next_y)));
        let mut dots = vec![vec![false; max_x + 1]; max_y + 1];
        points.for_each(|(x, y)| dots[y][x] = true);
        Paper {
            dots,
        }
    }

    fn fold(&mut self, fold_direction: &Fold, line: usize) {
        match fold_direction {
            Fold::Up => {
                let width = self.dots[0].len();
                self.dots.remove(line);
                let (top, bottom) = self.dots.split_at(line);
                let new_height = bottom.len().max(top.len());
                let mut dots = vec![vec![false; width]; new_height];
                for y in 0..new_height {
                    for x in 0..width {
                        dots[y][x] = bottom.get(new_height - 1 - y)
                                           .unwrap_or(&vec![false; width])[x] ||
                                     top.get(top.len() - new_height + y).unwrap_or(&vec![false; width])[x];
                    }
                }
                self.dots = dots;
            }
            Fold::Left => {
                let height = self.dots.len();
                let (left, right) = self.dots.iter_mut()
                    .map(|row| {
                        row.remove(line);
                        row
                    })
                    .map(|row| row.split_at(line))
                    .fold((vec![], vec![]), |(mut left, mut right), (next_left, next_right)| {
                        left.push(next_left);
                        right.push(next_right);
                        (left, right)
                    });
                let new_width = left[0].len().max(right[0].len());
                let mut dots = vec![vec![false; new_width]; height];
                for y in 0..height {
                    for x in 0..new_width {
                        dots[y][x] = *(right[y].get(new_width - 1 - x)
                                              .unwrap_or(&false)) ||
                                     *(left[y].get(left[0].len() - new_width + x).unwrap_or(&false));
                    }
                }
                self.dots = dots;
            }
        }
    }

    fn num_dots(&self) -> usize {
        self.dots.iter().map(|row| row.iter().filter(|dot| **dot).count()).sum()
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let (raw_paper, raw_folds) = contents.split_once("\n\n").unwrap();
    let mut paper = Paper::parse(raw_paper);
    let mut folds: Vec<(Fold, usize)> = raw_folds.split('\n')
        .map(|line| line[11..].split_once('=').unwrap())
        .map(|(axis, line)| (
            match axis {
                "x" => Fold::Left,
                "y" => Fold::Up,
                _ => panic!("Unknown axis"),
            },
            line.parse::<usize>().unwrap()
            )).collect();
    let (axis, line) = folds.remove(0);
    paper.fold(&axis, line);
    println!("After the first fold, there are {} dots", paper.num_dots());

    folds.iter().for_each(|(axis, line)| paper.fold(axis, *line));
    println!("After all the folds completed, the paper looks like this:\n{:?}", paper);
}

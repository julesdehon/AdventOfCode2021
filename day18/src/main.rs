use std::fs;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct SnailFishNumberElem {
    val: u32,
    depth: u32,
}

#[derive(Clone)]
struct SnailFishNumber {
    values: Vec<SnailFishNumberElem>
}

impl Add for &SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, rhs: Self) -> Self::Output {
        let mut num = SnailFishNumber {
            values: self.values.iter()
                .chain(rhs.values.iter())
                .map(|SnailFishNumberElem { val, depth }| {
                    SnailFishNumberElem { val: *val, depth: depth + 1 }
                })
                .collect()
        };
        while num.reduce() {}
        num
    }
}

impl SnailFishNumber {
    fn parse(s: &str) -> SnailFishNumber {
        let mut depth = -1;
        let mut values = vec![];
        let cs = s.chars();
        for c in cs {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => {},
                c => values.push(SnailFishNumberElem{val: c.to_digit(10).unwrap(), depth: depth as u32})
            }
        }
        SnailFishNumber {
            values,
        }
    }

    fn explode(&mut self) -> bool {
        for i in 0..self.values.len() {
            if self.values[i].depth < 4 { continue; }
            // values[i] and values[i + 1] are the pair
            if i > 0 {
                self.values[i - 1].val += self.values[i].val;
            }
            if i + 1 < self.values.len() - 1 {
                self.values[i + 2].val += self.values[i + 1].val;
            }
            self.values.remove(i);
            self.values[i] = SnailFishNumberElem{ val: 0, depth: self.values[i].depth - 1};
            return true
        }
        false
    }

    fn split(&mut self) -> bool {
        for i in 0..self.values.len() {
            let SnailFishNumberElem{ val, depth } = self.values[i];
            if val < 10 { continue; }
            self.values[i] = SnailFishNumberElem{ val: val / 2, depth: depth + 1 };
            self.values.insert(i + 1, SnailFishNumberElem{ val: val / 2 + val % 2, depth: depth + 1 });
            return true
        }
        false
    }

    fn reduce(&mut self) -> bool {
        self.explode() || self.split()
    }

    fn magnitude(&self) -> u32 {
        let mut vals = self.values.clone();
        while vals.len() > 1 {
            for i in 0..vals.len() - 1 {
                if vals[i].depth == vals[i + 1].depth {
                    vals[i] = SnailFishNumberElem {
                        val: 3 * vals[i].val + 2 * vals[i + 1].val,
                        depth: vals[i].depth - if vals[i].depth > 0 { 1 } else { 0 }
                    };
                    vals.remove(i + 1);
                    break;
                }
            }
        }
        vals[0].val
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let snail_fish_numbers = contents
        .split('\n')
        .map(SnailFishNumber::parse)
        .collect::<Vec<SnailFishNumber>>();

    let sum = snail_fish_numbers
        .clone()
        .into_iter()
        .reduce(|accum, next| &accum + &next)
        .unwrap()
        .magnitude();
    println!("Magnitude of the final sum is {}", sum);

    let largest_sum = part2(&snail_fish_numbers);
    println!("The largest magnitude of any sum of two different snailfish numbers is {}", largest_sum);
}

fn part2(snail_fish_numbers: &[SnailFishNumber]) -> u32 {
    let mut largest_sum = 0;
    for i in 0..snail_fish_numbers.len() {
        for j in 0..snail_fish_numbers.len() {
            if i == j { continue; }
            let sum = (&snail_fish_numbers[i] + &snail_fish_numbers[j]).magnitude();
            if sum > largest_sum {
                largest_sum = sum;
            }
        }
    }
    largest_sum
}

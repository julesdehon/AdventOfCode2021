use std::fs;

struct OctopusGrid {
    grid: Vec<Vec<u32>>,
}

impl OctopusGrid {
    fn parse(from: &str) -> OctopusGrid {
        let grid = from.split('\n').map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect();
        OctopusGrid {
            grid,
        }
    }

    fn pop(&mut self, i: usize, j: usize) {
        for p in if i > 0 { i - 1 } else { 0 }..=if i < self.grid.len() - 1 { i + 1 } else { self.grid.len() - 1 } {
            for q in if j > 0 { j - 1 } else { 0 }..=if j < self.grid[i].len() - 1 { j + 1 } else { self.grid[i].len() - 1 } {
                if i == p && j == q { continue }
                self.grid[p][q] += 1;
                if self.grid[p][q] == 10 {
                    self.pop(p, q);
                }
            }
        }
    }

    fn step(&mut self) -> u32 {
        self.grid = self.grid.iter().map(|line| line.iter().map(|x| x + 1).collect()).collect();
        let need_popping: Vec<(usize, usize)> = self.grid.iter().enumerate()
            .map(move |(i, line)|
                line.iter().enumerate().map(move |(j, energy)| (energy, (i, j)))
                                       .filter(|(energy, _)| **energy > 9)
                                       .map(|(_, (i, j))| (i, j)))
            .flatten()
            .collect();

        for (i, j) in need_popping {
            self.pop(i, j);
        }

        let num_flashes = self.grid.iter().fold(0, |num_flashes, line|
            num_flashes + line.iter().fold(0, |num_flashes, energy|
                if *energy > 9 { num_flashes + 1 } else { num_flashes }
            )
        );
        self.grid = self.grid.iter().map(|line| line.iter().map(|energy| if *energy > 9 { 0 } else { *energy }).collect()).collect();
        num_flashes
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let mut octopus_grid = OctopusGrid::parse(&contents);

    let num_flashes = part1(&mut octopus_grid);
    println!("After 100 steps, there were {} flashes", num_flashes);

    let first_simultaneous_flash = part2(&mut octopus_grid);
    println!("First simultaneous flash occurred after {} steps", first_simultaneous_flash);
}

fn part1(octopus_grid: &mut OctopusGrid) -> u32 {
    (0..100).fold(0, |num_flashes, _| num_flashes + octopus_grid.step())
}

fn part2(octopus_grid: &mut OctopusGrid) -> u32 {
    let mut result = 101;
    while octopus_grid.step() != 100 { result += 1 }
    result
}
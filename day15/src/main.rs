extern crate itertools;

use std::fs;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::{Ordering, Reverse};
use itertools::Itertools;

fn make_full_map(map_chunk: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut result = map_chunk.to_owned();
    for add in 1..5 {
        for i in 0..map_chunk.len() {
            result[i].append(&mut (map_chunk[i].iter().map(|n| 1 + ((n + add - 1) % 9)).collect()));
        }
    }
    for _ in 1..5 {
        for _ in 0..map_chunk.len() {
            result.push(result[result.len() - map_chunk.len()].iter().map(|n| 1 + (n % 9)).collect());
        }
    }
    result
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let risk_level_map: Vec<Vec<u32>> = contents
        .split('\n')
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();
    let total_risk = part1(&risk_level_map);
    println!("The lowest total risk from the top left to the bottom right is {}", total_risk);

    let full_risk_map = make_full_map(&risk_level_map);

    let total_risk_with_full_map = part1(&full_risk_map);
    println!("After realising the risk map is much bigger, the minimum risk path is {}", total_risk_with_full_map);
}

#[derive(Copy, Clone, Eq)]
struct PathElem {
    coord: (usize, usize),
    cost: u32,
}

impl Ord for PathElem {
    fn cmp(&self, other: &Self) -> Ordering {
        Reverse(self.cost).cmp(&Reverse(other.cost))
    }
}

impl PartialOrd for PathElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for PathElem {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

fn neighbours((x, y): (usize, usize), h: usize, w: usize) -> Vec<(usize, usize)> {
    let mut result = vec![];
    for p in 0.max(x as i32 - 1) as usize..=(w - 1).min(x + 1) {
        for q in 0.max(y as i32 -1) as usize..=(h - 1).min(y + 1) {
            if !(p == x || q == y) || (p == x && q == y) { continue; }
            result.push((p, q));
        }
    }
    result
}

// Solve with Dijkstra's algorithm!
fn part1(map: &[Vec<u32>]) -> u32 {
    let (h, w) = (map.len(), map[0].len());
    let mut dist: HashMap<_, _> = (0..h).cartesian_product(0..w)
        .map(|c| (c, u32::MAX)).collect();
    let mut pq = BinaryHeap::new();
    let start = (0, 0);
    let end = (w - 1, h - 1);

    dist.insert(start, 0);
    pq.push(PathElem { coord: (0, 0), cost: 0 });

    while let Some(PathElem { coord, cost }) = pq.pop() {
        if coord == end { return cost; }

        if cost > *dist.get(&coord).unwrap() { continue; }

        for (x, y) in neighbours(coord, h, w) {
            let next = PathElem { coord: (x, y), cost: cost + map[y][x] };

            if next.cost < *dist.get(&next.coord).unwrap() {
                pq.push(next);
                dist.insert(next.coord, next.cost);
            }
        }
    }

    panic!("Couldn't find the end!");
}

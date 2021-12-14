use std::fs;
use std::collections::HashSet;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let heights: Vec<Vec<u32>> = contents.split('\n').map(|line| line.chars().map(|char| char.to_digit(10).unwrap()).collect()).collect();

    let low_points = part1(&heights);
    println!("The sum of the risk levels of all the low points is {}", low_points.iter().fold(0, |sum, (i, j)| sum + heights[*i][*j] + 1));

    let multiplied_basin_sizes = part2(&heights, &low_points);
    println!("When you multiply together the szes of the three largest basins you get {}", multiplied_basin_sizes);
}

fn part1(heights: &[Vec<u32>]) -> Vec<(usize, usize)> {
    let mut low_points = vec![];
    for i in 0..heights.len() {
        for j in 0..heights[i].len() {
            let mut lowest = true;
            for p in if i == 0 { 0 } else { i - 1 }..=if i == heights.len() - 1 { heights.len() - 1 } else { i + 1 } {
                for q in if j == 0 { 0 } else { j - 1 }..=if j == heights[i].len() - 1 { heights[i].len() - 1 } else { j + 1 } {
                    if (p == i && q == j) || !(p == i || q == j) {
                        continue;
                    }
                    if heights[p][q] <= heights[i][j] {
                        lowest = false;
                    }
                }
            }
            if lowest {
                low_points.push((i, j));
            }
        }
    }
    low_points
}

fn part2(heights: &[Vec<u32>], low_points: &[(usize, usize)]) -> u32 {
    let mut in_basin = HashSet::new();
    let mut basin_sizes = vec![];
    for low_point in low_points {
        let basin_size = basin_search(heights, *low_point, &mut in_basin);
        if basin_size > 0 {
            basin_sizes.push(basin_size);
        }
    }
    basin_sizes.sort_unstable();
    return basin_sizes.iter().rev().take(3).product();
}

fn basin_search(heights: &[Vec<u32>], coord@(i, j): (usize, usize), already_in_basin: &mut HashSet<(usize, usize)>) -> u32 {
    if heights[i][j] == 9 || already_in_basin.contains(&(i, j)) {
        return 0;
    }
    already_in_basin.insert(coord);
    1 + if i > 0 { basin_search(heights, (i - 1, j), already_in_basin) } else { 0 }
      + if i < heights.len() - 1 { basin_search(heights, (i + 1, j), already_in_basin) } else { 0 }
      + if j > 0 { basin_search(heights, (i, j - 1), already_in_basin) } else { 0 }
      + if j < heights[i].len() - 1 { basin_search(heights, (i, j + 1), already_in_basin) } else { 0 }
}
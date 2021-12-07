use std::fs;
use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point(u32, u32);

fn parse_line(line: &str) -> (Point, Point) {
    let mut raw_points = line.split(" -> ");

    let mut from_coords = raw_points.next().unwrap().split(",");
    let from_point = Point(from_coords.next().unwrap().parse().unwrap(),
                           from_coords.next().unwrap().parse().unwrap());

    let mut to_coords = raw_points.next().unwrap().split(",");
    let to_point = Point(to_coords.next().unwrap().parse().unwrap(),
                         to_coords.next().unwrap().parse().unwrap());

    return (from_point, to_point);
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines = contents.split("\n");
    let segments = lines.map(parse_line).collect();

    let points_2_overlaps = part1(&segments);
    println!("There are {} points where at least two lines overlap", points_2_overlaps);

    let points_2_overlaps_with_diagonals = part2(&segments);
    println!("Considering diagonals also, there are {} points where at least two lines overlap", points_2_overlaps_with_diagonals);
}

fn part1(segments: &Vec<(Point, Point)>) -> u32 {
    let mut points_covered: HashMap<Point, u32> = HashMap::new();
    for segment in segments {
        let (p1, p2) = segment;
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        if !(x1 == x2 || y1 == y2) { continue }
        for x in if x1 <= x2 {*x1..=*x2} else {*x2..=*x1} {
            for y in if y1 <= y2 {*y1..=*y2} else {*y2..=*y1} {
                let line_count = points_covered.entry(Point(x, y)).or_insert(0);
                *line_count += 1;
            }
        }
    }
    return points_covered.iter().fold(0, |accum, (_, curr)| accum + if *curr > 1 { 1 } else { 0 });
}

fn part2(segments: &Vec<(Point, Point)>) -> u32 {
    let mut points_covered: HashMap<Point, u32> = HashMap::new();
    for segment in segments {
        let (p1, p2) = segment;
        let Point(x1, y1) = p1;
        let Point(x2, y2) = p2;
        if x1 == x2 || y1 == y2 {
            for x in if x1 <= x2 { *x1..=*x2 } else { *x2..=*x1 } {
                for y in if y1 <= y2 { *y1..=*y2 } else { *y2..=*y1 } {
                    let line_count = points_covered.entry(Point(x, y)).or_insert(0);
                    *line_count += 1;
                }
            }
        } else {
            let mut y: i32 = *y1 as i32;
            for x in if x1 <= x2 { (*x1..=*x2).collect::<Vec<u32>>() } else { (*x2..=*x1).rev().collect::<Vec<u32>>() } {
                let line_count = points_covered.entry(Point(x, y as u32)).or_insert(0);
                *line_count += 1;
                y = if y1 <= y2 { y + 1 } else { y - 1 };
            }
        }
    }
    return points_covered.iter().fold(0, |accum, (_, curr)| accum + if *curr > 1 { 1 } else { 0 });
}
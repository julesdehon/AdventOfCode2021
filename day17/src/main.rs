use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let x: Vec<Vec<i32>> = contents.strip_prefix("target area: ").unwrap()
        .split(", ")
        .map(|x| x[2..].split("..").map(|n| n.parse().unwrap()).collect())
        .collect();
    let ((x1, x2), (y1, y2)) = ((x[0][0], x[0][1]), (x[1][0], x[1][1]));

    let max_y_pos = part1(y1);
    println!("The maximum y position reached is {}", max_y_pos);

    let num_velocities = part2(x1, x2, y1, y2);
    println!("There are {} distinct initial velocity values that cause the probe to be within the target area after any step", num_velocities);
}

fn part1(y1: i32) -> i32 {
    let max_y_velocity = -y1 - 1;
    (max_y_velocity * (max_y_velocity + 1)) / 2
}

fn part2(x1: i32, x2: i32, y1: i32, y2: i32) -> i32 {
    let min_y_velocity = y1;
    let max_y_velocity = -y1 - 1;
    let min_x_velocity = (-0.5 + (2f64 * x1 as f64 + 0.25).sqrt()).floor() as i32;
    let max_x_velocity = x2;

    let mut num_velocities = 0;
    for vx in min_x_velocity..=max_x_velocity {
        for vy in min_y_velocity..=max_y_velocity {
            let (mut vvx, mut vvy) = (vx, vy);
            let (mut px, mut py) = (0, 0);
            while px <= x2 && py >= y1 {
                if px >= x1 && py <= y2 {
                    num_velocities += 1;
                    break;
                }
                px += vvx;
                py += vvy;
                if vvx > 0 {
                    vvx -= 1;
                }
                vvy -= 1;
            }
        }
    }
    num_velocities
}
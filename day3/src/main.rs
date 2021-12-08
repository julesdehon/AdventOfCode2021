use std::fs;
use std::cmp::Ordering;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let lines: Vec<&str> = contents.split('\n').collect();

    let (gamma_rate, epsilon_rate) = part1(&lines);
    println!("Gamma rate was {}, epsilon rate was {} - multiplying gives {}",
             gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);

    let (oxygen_generator_rating, co2_scrubber_rating) = part2(&lines);
    println!("Oxygen generator rating was {}, CO2 scrubber rating was {} - multiplying gives {}",
             oxygen_generator_rating, co2_scrubber_rating, oxygen_generator_rating * co2_scrubber_rating)
}

fn part1(lines: &[&str]) -> (i32, i32) {
    let mut gamma_rate = "".to_owned();
    let mut epsilon_rate = "".to_owned();
    for i in 0..lines[0].len() {
        let bits = lines.iter().map(|line| line.chars().nth(i).unwrap()).collect();
        let mcb = most_common_bit(bits, '1');
        gamma_rate.push(mcb);
        epsilon_rate.push(if mcb == '1' { '0' } else { '1' });
    }
    return (i32::from_str_radix(gamma_rate.as_str(), 2).unwrap(),
            i32::from_str_radix(epsilon_rate.as_str(), 2).unwrap());
}

fn part2(lines: &[&str]) -> (i32, i32) {
    let mut filtered_oxygen = lines.to_owned();
    let mut curr_bit_pos = 0;
    while filtered_oxygen.len() > 1 {
        let oxygen_bits = filtered_oxygen.iter().map(|bit_string| bit_string.chars().nth(curr_bit_pos).unwrap()).collect();
        let mcb = most_common_bit(oxygen_bits, '1');
        if filtered_oxygen.len() > 1 {
            filtered_oxygen.retain(|bit_string| bit_string.chars().nth(curr_bit_pos).unwrap() == mcb);
        }
        curr_bit_pos += 1;
    }

    let mut filtered_co2 = lines.to_owned();
    curr_bit_pos = 0;
    while filtered_co2.len() > 1 {
        let co2_bits = filtered_co2.iter().map(|bit_string| bit_string.chars().nth(curr_bit_pos).unwrap()).collect();
        let mcb = most_common_bit(co2_bits, '1'); // Set default to 1 since we are interested in least common bit
        if filtered_co2.len() > 1 {
            filtered_co2.retain(|bit_string| bit_string.chars().nth(curr_bit_pos).unwrap() == if mcb == '1' { '0' } else { '1' });
        }
        curr_bit_pos += 1;
    }

    (i32::from_str_radix(filtered_oxygen[0], 2).unwrap(),
     i32::from_str_radix(filtered_co2[0], 2).unwrap())
}

fn most_common_bit(bits: Vec<char>, default: char) -> char {
    let mut ones = 0;
    let mut zeros = 0;
    for bit in bits {
        match bit {
            '1' => { ones += 1 },
            '0' => { zeros += 1 },
            _ => { println!("Unexpected bit found") }
        }
    }
    match ones.cmp(&zeros) {
        Ordering::Less => '0',
        Ordering::Equal => default,
        Ordering::Greater => '1'
    }
}
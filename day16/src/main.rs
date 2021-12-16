use std::fs;

enum Operator {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl Operator {
    fn from_type_id(id: u32) -> Operator {
        match id {
            0 => Operator::Sum,
            1 => Operator::Product,
            2 => Operator::Minimum,
            3 => Operator::Maximum,
            5 => Operator::GreaterThan,
            6 => Operator::LessThan,
            7 => Operator::Equal,
            _ => unreachable!("Unknown type id"),
        }
    }
}

enum PacketContents {
    Value(u64),
    SubPackets(Operator, Vec<Packet>),
}

struct Packet {
    version_number: u32,
    contents: PacketContents,
}

impl Packet {
    fn parse(raw: &mut &str) -> Packet {
        let version_number = Packet::next(raw, 3);
        let packet_type = Packet::next(raw, 3);
        if packet_type == 4 {
            let mut value: u64 = 0;
            loop {
                let last_group = Packet::next(raw, 1) == 0;
                value = (value << 4) + Packet::next(raw, 4) as u64;
                if last_group { break; }
            }
            Packet {
                version_number,
                contents: PacketContents::Value(value),
            }
        } else {
            let mode = Packet::next(raw, 1);
            if mode == 0 {
                let sub_packets_length = Packet::next(raw, 15);
                let total_left_to_read = raw.len();
                let mut read_so_far = 0;
                let mut sub_packets = vec![];
                while (read_so_far as u32) < sub_packets_length {
                    sub_packets.push(Packet::parse(raw));
                    read_so_far = total_left_to_read - raw.len();
                }
                Packet {
                    version_number,
                    contents: PacketContents::SubPackets(Operator::from_type_id(packet_type),
                                                         sub_packets),
                }
            } else {
                let num_sub_packets = Packet::next(raw, 11);
                let mut sub_packets = vec![];
                for _ in 0..num_sub_packets {
                    sub_packets.push(Packet::parse(raw));
                }
                Packet {
                    version_number,
                    contents: PacketContents::SubPackets(Operator::from_type_id(packet_type),
                                                         sub_packets),
                }
            }
        }
    }

    fn next(binary_string: &mut &str, num_bits: usize) -> u32 {
        let result = u32::from_str_radix(&binary_string[..num_bits], 2)
            .expect("You didn't pass a binary number!");
        *binary_string = &binary_string[num_bits..];
        result
    }

    fn evaluate(&self) -> u64 {
        match &self.contents {
            PacketContents::Value(n) => *n as u64,
            PacketContents::SubPackets(operator, packets) => {
                match operator {
                    Operator::Sum => packets.iter().map(Packet::evaluate).sum(),
                    Operator::Product => packets.iter().map(Packet::evaluate).product(),
                    Operator::Minimum => packets.iter().map(Packet::evaluate).min().unwrap(),
                    Operator::Maximum => packets.iter().map(Packet::evaluate).max().unwrap(),
                    Operator::GreaterThan => {
                        if Packet::evaluate(&packets[0]) > Packet::evaluate(&packets[1]) {
                            1
                        } else { 0 }
                    },
                    Operator::LessThan => {
                        if Packet::evaluate(&packets[0]) < Packet::evaluate(&packets[1]) {
                            1
                        } else { 0 }
                    },
                    Operator::Equal => {
                        if Packet::evaluate(&packets[0]) == Packet::evaluate(&packets[1]) {
                            1
                        } else { 0 }
                    },
                }
            }
        }
    }
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");
    let raw: String = contents.chars().map(hex_to_binary).collect();
    let packet = Packet::parse(&mut raw.as_str());
    println!("Summed version numbers: {}", sum_version_numbers(&packet));
    println!("Packet evaluates to: {}", packet.evaluate());
}

fn sum_version_numbers(packet: &Packet) -> u32 {
    match &packet.contents {
        PacketContents::Value(_) => packet.version_number,
        PacketContents::SubPackets(_, packets) => packet.version_number + packets.iter().map(sum_version_numbers).sum::<u32>(),
    }
}

fn hex_to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}
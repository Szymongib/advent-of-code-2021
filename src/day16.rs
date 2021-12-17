use bitvec::prelude::*;
use std::fs;
use std::num::ParseIntError;
use std::ops::{Add, BitAnd, Shl};
use std::path::Path;

struct Packet {
    version: usize,
    content: PacketContent,
    packet_type: PacketType,
}

impl Packet {
    fn sum_versions(&self) -> usize {
        match &self.content {
            PacketContent::Literal(_) => self.version,
            PacketContent::SubPackets(sub_packets) => {
                self.version
                    + sub_packets
                        .iter()
                        .map(|sp| sp.sum_versions())
                        .sum::<usize>()
            }
        }
    }

    fn eval(&self) -> usize {
        match &self.content {
            PacketContent::Literal(val) => *val,
            PacketContent::SubPackets(sub_packets) => {
                let sub_packets_values: Vec<usize> =
                    sub_packets.iter().map(|sp| sp.eval()).collect();

                match &self.packet_type {
                    PacketType::Literal => unreachable!("unexpected literal as Operator packet"),
                    PacketType::Sum => sub_packets_values.iter().sum(),
                    PacketType::Product => sub_packets_values.iter().product(),
                    PacketType::Min => *(sub_packets_values.iter().min().unwrap()),
                    PacketType::Max => *(sub_packets_values.iter().max().unwrap()),
                    PacketType::Greater => (sub_packets_values[0] > sub_packets_values[1]).into(),
                    PacketType::Less => (sub_packets_values[0] < sub_packets_values[1]).into(),
                    PacketType::Equal => (sub_packets_values[0] == sub_packets_values[1]).into(),
                }
            }
        }
    }
}

enum PacketContent {
    Literal(usize),
    SubPackets(Vec<Packet>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum PacketType {
    Literal,
    Sum,
    Product,
    Min,
    Max,
    Greater,
    Less,
    Equal,
}

impl From<u8> for PacketType {
    fn from(num: u8) -> Self {
        match num {
            0 => PacketType::Sum,
            1 => PacketType::Product,
            2 => PacketType::Min,
            3 => PacketType::Max,
            4 => PacketType::Literal,
            5 => PacketType::Greater,
            6 => PacketType::Less,
            7 => PacketType::Equal,
            _ => panic!("unexpected operation number: {}", num),
        }
    }
}

pub fn day_16_1<P: AsRef<Path>>(input_file: P) -> usize {
    let top_packet = parse_top_packet(input_file);
    top_packet.sum_versions()
}

pub fn day_16_2<P: AsRef<Path>>(input_file: P) -> usize {
    let top_packet = parse_top_packet(input_file);
    top_packet.eval()
}

fn parse_top_packet<P: AsRef<Path>>(input_file: P) -> Packet {
    let top_packet = fs::read_to_string(input_file).expect("failed to read input");
    let bytes = decode_hex(&top_packet).expect("failed to decode hex");
    let bit_vec = BitVec::<Msb0, u8>::from_vec(bytes);
    let (packets, _) = process_packets(&bit_vec, 1);

    packets.into_iter().next().unwrap()
}

fn process_packets(bits: &BitSlice<Msb0, u8>, packets_limit: usize) -> (Vec<Packet>, usize) {
    let mut packets = vec![];

    let mut start = 0;
    loop {
        let version = bits_as_num(&bits[start..start + 3]);

        let packet_type_id = bits_as_num::<u8>(&bits[start + 3..start + 6]);
        let packet_type = PacketType::from(packet_type_id);

        let (packet_content, processed) = match packet_type {
            PacketType::Literal => process_literal_packet(&bits[start + 6..]),
            _ => process_operator_packet(&bits[start + 6..]),
        };
        start += 6 + processed;

        let packet = Packet {
            version,
            content: packet_content,
            packet_type,
        };

        packets.push(packet);

        if (bits.len() <= start) || packets_limit == packets.len() {
            break;
        }
    }

    (packets, start)
}

fn process_literal_packet(bits: &BitSlice<Msb0, u8>) -> (PacketContent, usize) {
    let mut start = 0;
    let mut end = 5;

    let mut value = bits_as_num::<usize>(&bits[start + 1..end]);

    while bits.len() > end && bits[start].bitand(true) {
        end += 5;
        start += 5;
        value <<= 4;
        value += bits_as_num::<usize>(&bits[start + 1..end]);
    }

    (PacketContent::Literal(value), end)
}

fn process_operator_packet(bits: &BitSlice<Msb0, u8>) -> (PacketContent, usize) {
    let (packets, processed) = if bits[0].bitand(true) {
        let packets_num = bits_as_num::<usize>(&bits[1..12]);
        let (packets, processed) = process_packets(&bits[12..], packets_num);
        (packets, processed + 12)
    } else {
        let to_process = bits_as_num::<usize>(&bits[1..16]);
        let (packets, processed) = process_packets(&bits[16..to_process + 16], usize::MAX);
        (packets, processed + 16)
    };

    (PacketContent::SubPackets(packets), processed)
}

fn bits_as_num<T: Default + Add<Output = T> + Shl<Output = T> + From<u8>>(
    bit_iter: &BitSlice<Msb0, u8>,
) -> T {
    let mut acc = T::default();
    for bit in bit_iter {
        acc = acc.shl(1.into());
        if bit.bitand(true) {
            acc = acc.add(1.into());
        }
    }
    acc
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[cfg(test)]
mod test {
    use crate::day16::{day_16_1, day_16_2};
    use crate::util::temp_file_with_content;

    #[test]
    fn test() {
        let test_cases = vec![
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ];

        for tc in test_cases {
            let file = temp_file_with_content("day_16_p1", tc.0);
            assert_eq!(day_16_1(&file), tc.1);
        }

        let test_cases = vec![
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ];

        for tc in test_cases.iter() {
            let file = temp_file_with_content("day_16_p2", tc.0);
            assert_eq!(day_16_2(&file), tc.1);
        }
    }
}

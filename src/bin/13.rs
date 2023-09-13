use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Clone, PartialEq, Eq)]
#[serde(untagged)]
enum PacketData {
    Value(u8),
    List(Vec<PacketData>),
}

impl fmt::Debug for PacketData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Value(v) => write!(f, "{v}"),
            Self::List(list) => f.debug_list().entries(list).finish(),
        }
    }
}

fn parse_packets(input: &str) -> Option<Vec<[PacketData; 2]>> {
    let mut packet_pairs: Vec<[PacketData; 2]> = vec![];
    let raw_lines = input.lines().collect::<Vec<&str>>();
    let mut raw_packet_pairs = raw_lines.chunks(3).clone();
    while let Some(raw_packet_pair) = raw_packet_pairs.next() {
        let packet0: Vec<PacketData> = serde_json::from_str(raw_packet_pair.get(0)?).ok()?;
        let packet1: Vec<PacketData> = serde_json::from_str(raw_packet_pair.get(1)?).ok()?;
        packet_pairs.push([PacketData::List(packet0), PacketData::List(packet1)]);
    }
    Some(packet_pairs)
}

fn validate(left: &PacketData, right: &PacketData) -> bool {
    match (left, right) {
        (PacketData::Value(left_value), PacketData::Value(right_value)) => {
            left_value < right_value
        }
        (PacketData::Value(left_value), PacketData::List(right_list)) => validate(
            &PacketData::List(vec![PacketData::Value(*left_value)]),
            &PacketData::List(right_list.clone()),
        ),
        (PacketData::List(left_list), PacketData::Value(right_value)) => validate(
            &PacketData::List(left_list.clone()),
            &PacketData::List(vec![PacketData::Value(*right_value)]),
        ),
        (PacketData::List(left_list), PacketData::List(right_list)) => {
            if left_list.len() == 0 {
                return true;
            }
            let mut validated = true;
            for (i, left_item) in left_list.iter().enumerate() {
                match right_list.get(i) {
                    None => {
                        break;
                    },
                    Some(right_item) => {
                        if validate(left_item, right_item) {
                            break;
                        } else {
                            validated = false;
                            break;
                        }
                    },
                }
            }
            validated
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let packet_pairs = parse_packets(input)?;

    let mut pairs_in_correct_order: Vec<usize> = vec![];
    for (i, packet_pair) in packet_pairs.iter().enumerate() {
        let [left_packet, right_packet] = packet_pair;
        if validate(left_packet, right_packet) {
            pairs_in_correct_order.push(i + 1);
        }
    }

    Some(pairs_in_correct_order.iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}

use advent_of_code::helpers::{parse_decimal, parse_int_decimal};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until, take_while},
    character::{is_newline, streaming::anychar},
    combinator::{map, map_res, rest},
    error::ErrorKind,
    multi::separated_list1,
    sequence::{pair, preceded},
};
use std::collections::VecDeque;

struct Monkey {
    /** Which monkey to throw to (if divisible, if not divisible) */
    throw_to: (u8, u8),
    check_divisible_by: u16,
    operation: fn(item: i32) -> i32,
    items: VecDeque<i32>,
}

#[derive(Debug)]
enum Operation {
    ADD(Operand),
    MULTIPLY(Operand),
}
#[derive(Debug)]
enum Operand {
    VALUE(i32),
    SELF,
}

fn parse_operation((operator_str, operand_str): (&str, &str)) -> Option<Operation> {
    let operand = match operand_str {
        "old" => Operand::SELF,
        _ => Operand::VALUE(operand_str.parse().ok()?),
    };
    let operation = match operator_str.chars().next() {
        Some('+') => Operation::ADD(operand),
        Some('*') => Operation::MULTIPLY(operand),
        _ => panic!("Operation can only be + or *"),
    };
    Some(operation)
}

fn parse_monkey(monkey_raw_input: &str) -> Option<Monkey> {
    let mut input_lines = monkey_raw_input.lines();
    input_lines.next(); // ignore first line
    let (_, starting_items) = preceded(
        tag("  Starting items: "),
        separated_list1(tag(", "), parse_int_decimal),
    )(input_lines.next()?)
    .ok()?;
    let (_, operation) = map(
        preceded(
            tag("  Operation: new = old "),
            pair(
                alt((tag("+ "), tag("* "))),
                alt((tag("old"), parse_decimal)),
            ),
        ),
        parse_operation,
    )(input_lines.next()?)
    .ok()?;

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut raw_monkeys = input.split("\n\n");
    parse_monkey(raw_monkeys.next()?);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}

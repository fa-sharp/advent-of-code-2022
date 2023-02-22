use advent_of_code::helpers::{parse_decimal_digits, parse_u64_decimal, parse_usize_decimal};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::separated_list1,
    sequence::{pair, preceded, terminated},
};
use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum Operation {
    ADD(Operand),
    MULTIPLY(Operand),
}
#[derive(Debug, Clone, Copy)]
enum Operand {
    VALUE(u64),
    SELF,
}
impl Operation {
    fn calc(&self, item: u64) -> u64 {
        match self {
            Operation::ADD(operand) => match operand {
                Operand::SELF => item + item,
                Operand::VALUE(value) => item + value,
            },
            Operation::MULTIPLY(operand) => match operand {
                Operand::SELF => item * item,
                Operand::VALUE(value) => item * value,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    /** which monkey to throw to: (if divisible, if not divisible) */
    throw_to: (usize, usize),
    check_divisible_by: u64,
    operation: Operation,
    num_items_inspected: u32,
    items: VecDeque<u64>,
}
struct MonkeyGroup {
    monkeys: Vec<Monkey>,
    should_worry_decline: bool,
    common_divisor: u64,
}
impl MonkeyGroup {
    fn new(monkeys: Vec<Monkey>, should_worry_decline: bool) -> Self {
        Self {
            should_worry_decline,
            common_divisor: monkeys
                .iter()
                .fold(1, |product, m| product * m.check_divisible_by),
            monkeys,
        }
    }

    fn round(&mut self) -> Option<()> {
        for monkey_idx in 0..self.monkeys.len() {
            // we'll need to clone the monkey in order to perform mutations safely
            let mut monkey = self.monkeys.get_mut(monkey_idx)?.clone();
            let mut items_inspected_count: u32 = 0;
            while let Some(mut current_item) = monkey.items.pop_front() {
                current_item %= self.common_divisor; // use the remainder to avoid overflows
                current_item = monkey.operation.calc(current_item); // do the monkey's operation
                if self.should_worry_decline {
                    current_item /= 3;
                };
                // check if divisible to determine which monkey to throw to
                let is_divisible = current_item % monkey.check_divisible_by == 0;
                let monkey_to_throw_to = match is_divisible {
                    true => self.monkeys.get_mut(monkey.throw_to.0),
                    false => self.monkeys.get_mut(monkey.throw_to.1),
                }?;
                monkey_to_throw_to.items.push_back(current_item);
                items_inspected_count += 1;
            }
            // clear the original monkey's items and update their # items inspected
            let orig_monkey = self.monkeys.get_mut(monkey_idx)?;
            orig_monkey.items.clear();
            orig_monkey.num_items_inspected += items_inspected_count;
        }
        Some(())
    }
}

fn parse_operation((operator_str, operand_str): (&str, &str)) -> Option<Operation> {
    let operand = match operand_str {
        "old" => Operand::SELF,
        value => Operand::VALUE(value.parse().ok()?),
    };
    let operation = match operator_str {
        "+" => Operation::ADD(operand),
        "*" => Operation::MULTIPLY(operand),
        _ => panic!("Operation can only be + or *"),
    };
    Some(operation)
}

fn parse_monkey(monkey_raw_input: &str) -> Option<Monkey> {
    let mut input_lines = monkey_raw_input.lines();
    input_lines.next(); // ignore first line
    let (_, starting_items) = preceded(
        preceded(complete::space1, tag("Starting items: ")),
        separated_list1(tag(", "), parse_u64_decimal),
    )(input_lines.next()?)
    .ok()?;
    let (_, operation) = map(
        preceded(
            preceded(complete::space1, tag("Operation: new = old ")),
            pair(
                terminated(alt((tag("+"), tag("*"))), tag(" ")),
                alt((tag("old"), parse_decimal_digits)),
            ),
        ),
        parse_operation,
    )(input_lines.next()?)
    .ok()?;
    let (_, divisible_by) = preceded(
        preceded(complete::space1, tag("Test: divisible by ")),
        parse_u64_decimal,
    )(input_lines.next()?)
    .ok()?;
    let (_, throw_to_true) = preceded(
        preceded(complete::space1, tag("If true: throw to monkey ")),
        parse_usize_decimal,
    )(input_lines.next()?)
    .ok()?;
    let (_, throw_to_false) = preceded(
        preceded(complete::space1, tag("If false: throw to monkey ")),
        parse_usize_decimal,
    )(input_lines.next()?)
    .ok()?;
    Some(Monkey {
        items: starting_items.into(),
        num_items_inspected: 0,
        check_divisible_by: divisible_by,
        throw_to: (throw_to_true, throw_to_false),
        operation: operation?,
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkey_group =
        MonkeyGroup::new(input.split("\n\n").map_while(parse_monkey).collect(), true);
    for _ in 0..20 {
        monkey_group.round();
    }
    let mut sorted_monkeys = monkey_group.monkeys.clone();
    sorted_monkeys.sort_by(|a, b| b.num_items_inspected.cmp(&a.num_items_inspected));
    let monkey_business = sorted_monkeys
        .iter()
        .take(2)
        .fold(1, |monkey_business, monkey| {
            monkey_business * monkey.num_items_inspected
        });
    Some(monkey_business)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut monkey_group =
        MonkeyGroup::new(input.split("\n\n").map_while(parse_monkey).collect(), false);
    for _ in 0..10000 {
        monkey_group.round();
    }
    let mut sorted_monkeys = monkey_group.monkeys.clone();
    sorted_monkeys.sort_by(|a, b| b.num_items_inspected.cmp(&a.num_items_inspected));
    let monkey_business: u64 = sorted_monkeys
        .iter()
        .take(2)
        .fold(1, |monkey_business, monkey| {
            monkey_business * monkey.num_items_inspected as u64
        });
    Some(monkey_business)
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
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

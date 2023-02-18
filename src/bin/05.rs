use std::collections::{HashMap, VecDeque};

pub fn part_one(input: &str) -> Option<String> {
    let (raw_stacks, raw_instructions) = input.split_once("\n\n").unwrap();

    // Create a hashmap representing the stacks. Each stack is stored as a 'VecDeque' so
    // we can easily add/remove from the front and back as needed
    let mut stacks = create_stack_hashmap(raw_stacks);

    // Parse the instructions and move the stacks
    let instructions = parse_instructions(raw_instructions);
    for (num_crates_to_move, from_stack_id, to_stack_id) in instructions {
        for _ in 0..num_crates_to_move {
            let moved_crate = stacks.get_mut(&from_stack_id).unwrap().pop_front().unwrap();
            stacks
                .get_mut(&to_stack_id)
                .unwrap()
                .push_front(moved_crate);
        }
    }

    // get the top crate of every stack and return it as a string
    let mut top_crates = String::new();
    for stack_id in 1_u8..=stacks.len().try_into().unwrap() {
        let stack = stacks.get_mut(&stack_id).unwrap();
        top_crates.push(stack.pop_front().unwrap_or_default());
    }

    Some(top_crates)
}

pub fn part_two(input: &str) -> Option<String> {
    let (raw_stacks, raw_instructions) = input.split_once("\n\n").unwrap();

    let mut stacks = create_stack_hashmap(raw_stacks);
    let instructions = parse_instructions(raw_instructions);

    // We'll move the stacks as a group this time!
    for (num_crates_to_move, from_stack_id, to_stack_id) in instructions {
        let mut crates_to_move: Vec<char> = vec![];
        for _ in 0..num_crates_to_move {
            let moved_crate = stacks.get_mut(&from_stack_id).unwrap().pop_front().unwrap();
            crates_to_move.push(moved_crate)
        }
        for _ in 0..num_crates_to_move {
            stacks
                .get_mut(&to_stack_id)
                .unwrap()
                .push_front(crates_to_move.pop().unwrap());
        }
    }

    // get the top crate of every stack and return it as a string
    let mut top_crates = String::new();
    for stack_id in 1_u8..=stacks.len().try_into().unwrap() {
        let stack = stacks.get_mut(&stack_id).unwrap();
        top_crates.push(stack.pop_front().unwrap_or_default());
    }

    Some(top_crates)
}

fn create_stack_hashmap(raw_stacks: &str) -> HashMap<u8, VecDeque<char>> {
    let mut stacks: HashMap<u8, VecDeque<char>> = HashMap::new();
    for stack_row in raw_stacks.split('\n') {
        for (stack_num, raw_crate) in stack_row
            .chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
        {
            let crate_char = raw_crate[1];
            if crate_char >= 'A' && crate_char <= 'Z' {
                let stack = stacks
                    .entry((stack_num + 1).try_into().unwrap())
                    .or_default();
                stack.push_back(raw_crate[1]);
            }
        }
    }
    stacks
}

fn parse_instructions(raw_instructions: &str) -> Vec<(u32, u8, u8)> {
    let mut instructions: Vec<(u32, u8, u8)> = vec![];
    for raw_instruction in raw_instructions.split('\n') {
        let mut instr_parts = raw_instruction
            .strip_prefix("move ")
            .unwrap()
            .split(" from ");
        let num_crates_to_move: u32 = instr_parts
            .next()
            .unwrap()
            .parse()
            .expect("Couldn't parse # crates");
        let mut raw_stack_ids = instr_parts.next().unwrap().split(" to ");
        let from_stack_id: u8 = raw_stack_ids
            .next()
            .unwrap()
            .parse()
            .expect("Couldn't parse 'from' stack");
        let to_stack_id: u8 = raw_stack_ids
            .next()
            .unwrap()
            .parse()
            .expect("Couldn't parse 'to' stack");

        instructions.push((num_crates_to_move, from_stack_id, to_stack_id));
    }
    instructions
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}

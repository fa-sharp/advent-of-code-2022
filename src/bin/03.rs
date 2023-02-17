use std::collections::HashSet;

fn get_priority(c: char) -> Option<u32> {
    let ascii_value = c as u32;
    if ascii_value >= 65 && ascii_value <= 90 {
        return Some(ascii_value - 38);
    }
    if ascii_value >= 97 && ascii_value <= 122 {
        return Some(ascii_value - 96);
    }
    None
}

fn find_common_item_two(first_rucksack: &str, second_rucksack: &str) -> Option<char> {
    let item_hashset: HashSet<char> = first_rucksack.chars().collect();
    for item in second_rucksack.chars() {
        if item_hashset.contains(&item) {
            return Some(item);
        }
    }
    None
}
fn find_common_item_three(
    first_rucksack: &str,
    second_rucksack: &str,
    third_rucksack: &str,
) -> Option<char> {
    let first_hashset: HashSet<char> = first_rucksack.chars().collect();
    let second_hashset: HashSet<char> = second_rucksack.chars().collect();
    let common_items: HashSet<&char> = first_hashset.intersection(&second_hashset).collect();
    for item in third_rucksack.chars() {
        if common_items.contains(&item) {
            return Some(item);
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_priority = 0_u32;
    for input_line in input.split('\n') {
        let (first_rucksack, second_rucksack) = input_line.split_at(input_line.len() / 2);
        let common_item =
            find_common_item_two(first_rucksack, second_rucksack).expect("No common item found!");
        total_priority += get_priority(common_item).expect("Couldn't calculate priority.");
    }
    Some(total_priority)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_priority = 0_u32;
    let input_lines: Vec<&str> = input.split('\n').collect();
    for rucksacks in input_lines.chunks(3) {
        let common_item = find_common_item_three(rucksacks[0], rucksacks[1], rucksacks[2])
            .expect("No common item found!");
        total_priority += get_priority(common_item).expect("Couldn't calculate priority.");
    }
    Some(total_priority)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

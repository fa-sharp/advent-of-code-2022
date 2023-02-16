use std::collections::BinaryHeap;

/** Find largest calorie count. Using basic sort (boring!) */
pub fn part_one(input: &str) -> Option<u32> {
    let mut elves: Vec<u32> = vec![];
    for elf_calories in input.split("\n\n") {
        let mut total_calories = 0_u32;
        for calories in elf_calories.split('\n') {
            total_calories += calories.parse::<u32>().expect("Failed to parse calories");
        }
        elves.push(total_calories)
    }
    elves.sort();

    Some(elves[elves.len() - 1])
}

/** Find three largest calorie counts. Using binary heap this time! */
pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_heap: BinaryHeap<u32> = BinaryHeap::new();
    for elf_calories in input.split("\n\n") {
        let mut total_calories = 0_u32;
        for calories in elf_calories.split('\n') {
            total_calories += calories.parse::<u32>().expect("Failed to parse calories");
        }
        elf_heap.push(total_calories)
    }

    let top_three_sum =
        elf_heap.pop().unwrap_or(0) + elf_heap.pop().unwrap_or(0) + elf_heap.pop().unwrap_or(0);

    Some(top_three_sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

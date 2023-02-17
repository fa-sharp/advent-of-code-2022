struct ElfRange {
    start: u32,
    end: u32,
}

fn get_parsed_range(range_input: &str) -> Option<ElfRange> {
    if let Some((start_str, end_str)) = range_input.split_once('-') {
        let parsed_start = start_str.parse::<u32>();
        let parsed_end = end_str.parse::<u32>();
        return match (parsed_start, parsed_end) {
            (Ok(start), Ok(end)) => Some(ElfRange { start, end }),
            _ => None,
        };
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut num_fully_contained = 0;
    for elf_pair in input.split('\n') {
        let (range_1, range_2) = elf_pair.split_once(',').unwrap();
        let elf_1 = get_parsed_range(range_1).expect("Couldn't parse range");
        let elf_2 = get_parsed_range(range_2).expect("Couldn't parse range");
        if (elf_1.start <= elf_2.start && elf_1.end >= elf_2.end)
            || (elf_2.start <= elf_1.start && elf_2.end >= elf_1.end)
        {
            num_fully_contained += 1;
        }
    }
    Some(num_fully_contained)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut num_partially_contained = 0;
    for elf_pair in input.split('\n') {
        let (range_1, range_2) = elf_pair.split_once(',').unwrap();
        let elf_1 = get_parsed_range(range_1).expect("Couldn't parse range");
        let elf_2 = get_parsed_range(range_2).expect("Couldn't parse range");
        if (elf_1.start <= elf_2.start && elf_1.end >= elf_2.start)
            || (elf_2.start <= elf_1.start && elf_2.end >= elf_1.start)
        {
            num_partially_contained += 1;
        }
    }
    Some(num_partially_contained)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

// Thanks to fasterthanlime for this idea using bitwise operations: https://fasterthanli.me/series/advent-of-code-2022/part-6

pub fn part_one(input: &str) -> Option<usize> {
    // We want to identify the first 'marker' i.e. the first consecutive sequence of four different characters
    let first_marker_index = input
        .as_bytes()
        .windows(4) // look at windows of four characters
        .map(|window| {
            window
                .iter()
                .map(|c| {
                    assert!(c.is_ascii_lowercase()); // all letters should be ASCII lowercase
                    (1 << (*c as u32 - 'a' as u32)) as u32 // transform the letters into bits: a = 1, b = 10, c = 100, etc.
                })
                .fold(0, |acc, bits| acc | bits) // calculate sum of bitwise OR: 'aabb' = 11, 'abcd' = 1111, 'abce' = 10111
        })
        .position(|sum_of_bitwise_or: u32| sum_of_bitwise_or.count_ones() == 4) // a sequence of four different letters will be indicated by four 1's
        .expect("Didn't find a consecutive sequence of four different letters!");
    Some(first_marker_index + 4)
}

pub fn part_two(input: &str) -> Option<usize> {
    // Same idea as above, except now we're looking for a sequence of 14 distinct characters
    let first_marker_index = input
        .as_bytes()
        .windows(14)
        .map(|window| {
            window
                .iter()
                .map(|c| {
                    assert!(c.is_ascii_lowercase());
                    (1 << (*c as u32 - 'a' as u32)) as u32
                })
                .fold(0, |acc, bits| acc | bits)
        })
        .position(|sum_of_bitwise_or: u32| sum_of_bitwise_or.count_ones() == 14)
        .expect("Didn't find a consecutive sequence of 14 different letters!");
    Some(first_marker_index + 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(10));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(29));
    }
}

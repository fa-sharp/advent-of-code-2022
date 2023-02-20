use std::collections::HashSet;

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}
impl Coord {
    fn step(&mut self, direction: &Direction) {
        match direction {
            Direction::UP => self.y += 1,
            Direction::DOWN => self.y -= 1,
            Direction::LEFT => self.x -= 1,
            Direction::RIGHT => self.x += 1,
        }
    }
}

fn parse_instructions(input: &str) -> Vec<(Direction, u8)> {
    let mut instructions: Vec<(Direction, u8)> = vec![];
    for line in input.lines() {
        let (raw_direction, raw_steps) = line
            .split_once(' ')
            .expect("Should be a space in each instruction");
        let direction = match raw_direction {
            "U" => Direction::UP,
            "R" => Direction::RIGHT,
            "L" => Direction::LEFT,
            "D" => Direction::DOWN,
            _ => panic!("Unrecognized direction!"),
        };
        instructions.push((direction, raw_steps.parse().expect("Should be integer")));
    }
    instructions
}

/// Helper function to move the rope tail so that it follows the head
fn move_tail(tail: &mut Coord, head: &Coord) {
    let dx = head.x.abs_diff(tail.x);
    let dy = head.y.abs_diff(tail.y);
    if dx == 2 && dy == 2 {
        match head.x > tail.x {
            true => tail.x += 1,
            false => tail.x -= 1,
        };
        match head.y > tail.y {
            true => tail.y += 1,
            false => tail.y -= 1,
        }
    } else if dx == 2 {
        tail.y = head.y;
        match head.x > tail.x {
            true => tail.x += 1,
            false => tail.x -= 1,
        };
    } else if dy == 2 {
        tail.x = head.x;
        match head.y > tail.y {
            true => tail.y += 1,
            false => tail.y -= 1,
        };
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let instructions = parse_instructions(input);
    let mut visited_coords: HashSet<Coord> = HashSet::new();

    let mut head = Coord { x: 0, y: 0 };
    let mut tail = Coord { x: 0, y: 0 };

    for (dir, steps) in instructions {
        for _ in 0..steps {
            head.step(&dir);
            move_tail(&mut tail, &head);
            visited_coords.insert(tail.clone());
        }
    }

    Some(visited_coords.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let instructions = parse_instructions(input);
    let mut visited_coords: HashSet<Coord> = HashSet::new();

    let mut head = Coord { x: 0, y: 0 };
    let mut tails = [head.clone(); 9];

    for (dir, steps) in instructions {
        for _ in 0..steps {
            head.step(&dir);
            let mut prev_tail = &head;
            for tail in tails.iter_mut() {
                move_tail(tail, prev_tail);
                prev_tail = tail;
            }
            visited_coords.insert(tails.last().unwrap().clone());
        }
    }

    Some(visited_coords.len())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), Some(1));
    }
}

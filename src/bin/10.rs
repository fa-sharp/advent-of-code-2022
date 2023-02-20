use std::collections::VecDeque;

enum Instruction {
    NoOp,
    AddX(i32),
}

struct Cpu {
    /** the overall cycle count */
    cycle: u32,
    /** the current value in the register */
    register_value: i32,
    /** the currently loaded instruction */
    current_instruction: Instruction,
    /** cycle count for the current instruction */
    current_cycle: u8,
    /** a queue of CPU instructions */
    instructions: VecDeque<Instruction>,
}

impl Cpu {
    fn new(mut instructions: VecDeque<Instruction>) -> Self {
        let current_instruction = instructions
            .pop_front()
            .expect("Should have at least 1 instruction!");
        Self {
            cycle: 0,
            register_value: 1,
            current_instruction,
            current_cycle: 0,
            instructions,
        }
    }
    fn load_next_instruction(&mut self) {
        self.current_cycle = 0;
        self.current_instruction = self
            .instructions
            .pop_front()
            .expect("Should have next instruction!");
    }
    fn simulate_cycle(&mut self) {
        match self.current_instruction {
            Instruction::NoOp => {
                if self.current_cycle == 1 {
                    self.load_next_instruction();
                }
            }
            Instruction::AddX(x) => {
                if self.current_cycle == 2 {
                    self.register_value += x;
                    self.load_next_instruction();
                }
            }
        }
        self.cycle += 1;
        self.current_cycle += 1;
    }
    fn simulate_cycles(&mut self, num_cycles: u32) {
        for _ in 0..num_cycles {
            self.simulate_cycle();
        }
    }
    fn calc_signal_strength(&self) -> i32 {
        self.register_value * self.cycle as i32
    }
}

fn parse_cpu_instructions(input: &str) -> VecDeque<Instruction> {
    let mut instructions: VecDeque<Instruction> = VecDeque::new();
    for line in input.lines() {
        match line.get(0..4) {
            Some("noop") => instructions.push_back(Instruction::NoOp),
            Some("addx") => {
                let (_, value) = line
                    .split_once(' ')
                    .expect("Should be space in this instruction");
                instructions.push_back(Instruction::AddX(
                    value.parse().expect("Addx value should be integer"),
                ))
            }
            _ => panic!("Unrecognized instruction!"),
        }
    }
    instructions
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = Cpu::new(parse_cpu_instructions(input));
    cpu.simulate_cycles(20);
    let mut signal_strength = cpu.calc_signal_strength();
    for _ in 0..5 {
        cpu.simulate_cycles(40);
        signal_strength += cpu.calc_signal_strength();
    }
    Some(signal_strength)
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

fn get_sprite_positions(register_value: i32, vertical_pos: i32) -> [Coord; 3] {
    let coord = Coord {
        x: register_value,
        y: vertical_pos,
    };
    [
        Coord {
            x: coord.x - 1,
            y: coord.y,
        },
        coord,
        Coord {
            x: coord.x + 1,
            y: coord.y,
        },
    ]
}

pub fn part_two(input: &str) -> Option<String> {
    let mut cpu = Cpu::new(parse_cpu_instructions(input));
    let mut output = String::new();

    for vertical_pos in 0..6 {
        for horizontal_pos in 0..40 {
            cpu.simulate_cycle();
            let current_position = Coord {
                x: horizontal_pos,
                y: vertical_pos,
            };
            let sprite_positions = get_sprite_positions(cpu.register_value, vertical_pos);
            if sprite_positions.contains(&current_position) {
                output += "#";
            } else {
                output += ".";
            }
        }
        output += "\n";
    }

    Some(output)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(
                r#"##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"#
                .to_string()
            )
        );
    }
}

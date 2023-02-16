#[derive(PartialEq, Eq)]
enum MOVE {
    Rock,
    Paper,
    Scissors,
}

impl MOVE {
    fn will_beat(&self, opp_move: MOVE) -> bool {
        match self {
            MOVE::Rock => opp_move == MOVE::Scissors,
            MOVE::Paper => opp_move == MOVE::Rock,
            MOVE::Scissors => opp_move == MOVE::Paper,
        }
    }
    fn get_winning_move(&self) -> MOVE {
        match self {
            MOVE::Scissors => MOVE::Rock,
            MOVE::Rock => MOVE::Paper,
            MOVE::Paper => MOVE::Scissors,
        }
    }
    fn get_losing_move(&self) -> MOVE {
        match self {
            MOVE::Rock => MOVE::Scissors,
            MOVE::Paper => MOVE::Rock,
            MOVE::Scissors => MOVE::Paper,
        }
    }
    fn value(&self) -> u32 {
        match self {
            MOVE::Rock => 1,
            MOVE::Paper => 2,
            MOVE::Scissors => 3,
        }
    }
}

fn calc_score(my_move: MOVE, opp_move: MOVE) -> u32 {
    if my_move == opp_move {
        my_move.value() + 3
    } else if my_move.will_beat(opp_move) {
        my_move.value() + 6
    } else {
        my_move.value()
    }
}

fn input_to_move(c: &str) -> Option<MOVE> {
    match c {
        "A" | "X" => Some(MOVE::Rock),
        "B" | "Y" => Some(MOVE::Paper),
        "C" | "Z" => Some(MOVE::Scissors),
        _ => None,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total_score = 0_u32;
    for round in input.split('\n') {
        if let Some((opp_input, player_input)) = round.split_once(' ') {
            let opp_move = input_to_move(opp_input).expect("Failed to parse opponent move");
            let my_move = input_to_move(player_input).expect("Failed to parse player move");
            let score = calc_score(my_move, opp_move);
            total_score += score;
        }
    }

    Some(total_score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total_score = 0_u32;
    for round in input.split('\n') {
        if let Some((opp_input, outcome)) = round.split_once(' ') {
            let opp_move = input_to_move(opp_input).expect("Failed to parse opponent move");
            match outcome {
                "X" => {
                    // need to lose
                    total_score += calc_score(opp_move.get_losing_move(), opp_move)
                }
                "Y" => {
                    // need to draw
                    total_score += opp_move.value() + 3
                }
                "Z" => {
                    // need to win
                    total_score += calc_score(opp_move.get_winning_move(), opp_move)
                }
                _ => panic!("Didn't recognize outcome"),
            }
        }
    }

    Some(total_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

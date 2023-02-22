use std::collections::{HashMap, HashSet};

use advent_of_code::grid::{Grid, GridCoord};

#[derive(Debug, Clone, Hash, Copy, PartialEq, Eq)]
enum Square {
    Start(u8),
    End(u8),
    Normal(u8),
}
impl Default for Square {
    fn default() -> Self {
        Self::Normal(0)
    }
}
impl Square {
    fn elev(&self) -> u8 {
        match self {
            Square::Start(elev) => *elev,
            Square::End(elev) => *elev,
            Square::Normal(elev) => *elev,
        }
    }
}

fn parse_grid(input: &str) -> Option<Grid<Square>> {
    let mut input_lines = input.lines().peekable();
    let mut grid = Grid::new(input_lines.peek()?.chars().count(), input_lines.count());

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let square = match char {
                'S' => Square::Start(0),
                'E' => Square::End(25),
                char => Square::Normal(char as u8 - 'a' as u8),
            };
            grid.insert_cell((x, y).into(), square)?;
        }
    }
    Some(grid)
}

fn find_walkable_neighbors(grid: &Grid<Square>, coord: &GridCoord) -> Vec<GridCoord> {
    let elevation = grid.get_cell(coord).expect("cell should exist").elev();
    let deltas = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    deltas
        .into_iter()
        .filter_map(|(dx, dy)| {
            Some(GridCoord {
                x: coord.x.checked_add_signed(dx)?,
                y: coord.y.checked_add_signed(dy)?,
            })
            .filter(|coord| {
                grid.is_in_bounds(coord) && {
                    let other_elevation = grid.get_cell(coord).expect("should be in bounds").elev();
                    other_elevation <= elevation + 1
                }
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_grid(input)?;
    let mut visited_coords: HashMap<GridCoord, Option<GridCoord>> = HashMap::new();
    let mut current_coords: HashSet<GridCoord> = HashSet::new();

    let start_coords = grid
        .iter_all_cells()
        .find(|(_, square)| **square == Square::Start(0))?
        .0;
    let end_coords = grid
        .iter_all_cells()
        .find(|(_, square)| **square == Square::End(25))?
        .0;
    current_coords.insert(start_coords.clone());
    visited_coords.insert(start_coords.clone(), None);

    let mut num_steps = 0;
    while !visited_coords.contains_key(end_coords) {
        let mut next_coords: HashSet<GridCoord> = HashSet::new();
        for current_coord in &current_coords {
            for new_coord in find_walkable_neighbors(&grid, current_coord) {
                if visited_coords.contains_key(&new_coord) {
                    continue;
                }
                visited_coords.insert(new_coord, Some(current_coord.clone()));
                next_coords.insert(new_coord);
            }
        }
        current_coords = next_coords;
        num_steps += 1;
    }

    Some(num_steps)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}

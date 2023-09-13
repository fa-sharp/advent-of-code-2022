use advent_of_code::grid::{Grid, GridCoord};

#[derive(Default, Clone, PartialEq, Eq)]
enum Tile {
    #[default]
    Air,
    Rock,
    Sand,
}

struct Cave {
    tiles: Grid<Tile>,
    bottom_row: usize,
}

impl Cave {
    fn new(tiles: Grid<Tile>) -> Self {
        let (deepest_rock_coord, _) = tiles
            .iter_all_cells()
            .filter(|(_, tile)| **tile == Tile::Rock)
            .max_by_key(|(coord, _)| coord.y)
            .unwrap();
        let bottom_row = deepest_rock_coord.y;
        Self { tiles, bottom_row }
    }

    fn new_part_2(mut tiles: Grid<Tile>) -> Self {
        let (deepest_rock_coord, _) = tiles
            .iter_all_cells()
            .filter(|(_, tile)| **tile == Tile::Rock)
            .max_by_key(|(coord, _)| coord.y)
            .unwrap();

        // fill the bottom row with rocks
        let bottom_row = deepest_rock_coord.y + 2;
        let bottom_row_coords: Vec<GridCoord> = tiles
            .iter_all_cells()
            .filter_map(|(coord, _)| {
                if coord.y == bottom_row {
                    Some(coord.clone())
                } else {
                    None
                }
            })
            .collect();
        for coord in bottom_row_coords {
            *tiles.get_cell_mut(&coord).unwrap() = Tile::Rock;
        }

        Self { tiles, bottom_row }
    }

    fn release_sand(&mut self) -> bool {
        let mut pos = GridCoord::from((500, 0));
        loop {
            // check if sand has fallen past last row
            if pos.y >= self.bottom_row {
                return true;
            }

            // simulate falling sand
            let bottom_cell = GridCoord {
                x: pos.x,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_cell) {
                if *tile == Tile::Air {
                    pos = bottom_cell;
                    continue;
                }
            }
            let bottom_left_cell = GridCoord {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_left_cell) {
                if *tile == Tile::Air {
                    pos = bottom_left_cell;
                    continue;
                }
            }
            let bottom_right_cell = GridCoord {
                x: pos.x + 1,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_right_cell) {
                if *tile == Tile::Air {
                    pos = bottom_right_cell;
                    continue;
                }
            }

            // if we've reached this point, sand will rest in this position
            *self.tiles.get_cell_mut(&pos).expect("cell out of bounds") = Tile::Sand;
            return false;
        }
    }

    fn release_sand_part_2(&mut self) -> bool {
        let mut pos = GridCoord::from((500, 0));
        loop {
            // simulate falling sand
            let bottom_cell = GridCoord {
                x: pos.x,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_cell) {
                if *tile == Tile::Air {
                    pos = bottom_cell;
                    continue;
                }
            }
            let bottom_left_cell = GridCoord {
                x: pos.x - 1,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_left_cell) {
                if *tile == Tile::Air {
                    pos = bottom_left_cell;
                    continue;
                }
            }
            let bottom_right_cell = GridCoord {
                x: pos.x + 1,
                y: pos.y + 1,
            };
            if let Some(tile) = self.tiles.get_cell(&bottom_right_cell) {
                if *tile == Tile::Air {
                    pos = bottom_right_cell;
                    continue;
                }
            }

            // if we've reached this point, sand will rest in this position. check if we've reached the source/top
            if pos == GridCoord::from((500, 0)) {
                return true;
            }

            *self.tiles.get_cell_mut(&pos).expect("cell out of bounds") = Tile::Sand;
            return false;
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut cave = Cave::new(parse_tiles(input));
    let mut num_sand: u32 = 0;

    loop {
        let is_falling_into_abyss = cave.release_sand();
        if is_falling_into_abyss {
            break;
        }
        num_sand += 1;
    }

    Some(num_sand)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cave = Cave::new_part_2(parse_tiles(input));
    let mut num_sand: u32 = 0;

    loop {
        num_sand += 1;
        let has_reached_top = cave.release_sand_part_2();
        if has_reached_top {
            break;
        }
    }

    Some(num_sand)
}

fn parse_tiles(input: &str) -> Grid<Tile> {
    let mut grid: Grid<Tile> = Grid::new(1000, 1000);
    for path in input.lines() {
        let raw_points = path.split(" -> ").collect::<Vec<&str>>();
        for line in raw_points.windows(2) {
            let raw_point_a = line.get(0).unwrap().split_once(",").unwrap();
            let raw_point_b = line.get(1).unwrap().split_once(",").unwrap();
            let point_a = GridCoord {
                x: raw_point_a.0.parse().unwrap(),
                y: raw_point_a.1.parse().unwrap(),
            };
            let point_b = GridCoord {
                x: raw_point_b.0.parse().unwrap(),
                y: raw_point_b.1.parse().unwrap(),
            };
            let dx: i16 = i16::try_from(point_b.x).unwrap() - i16::try_from(point_a.x).unwrap();
            let dy: i16 = i16::try_from(point_b.y).unwrap() - i16::try_from(point_a.y).unwrap();
            let mut current_point = point_a.clone();
            loop {
                *grid
                    .get_cell_mut(&current_point)
                    .expect("cell out of bounds") = Tile::Rock;
                if current_point == point_b {
                    break;
                }
                current_point.x = {
                    if dx > 0 {
                        current_point.x + 1
                    } else if dx < 0 {
                        current_point.x - 1
                    } else {
                        current_point.x
                    }
                };
                current_point.y = {
                    if dy > 0 {
                        current_point.y + 1
                    } else if dy < 0 {
                        current_point.y - 1
                    } else {
                        current_point.y
                    }
                };
            }
        }
    }

    grid
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one(&input), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two(&input), Some(93));
    }
}

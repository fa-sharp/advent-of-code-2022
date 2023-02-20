use advent_of_code::grid::{Grid, GridCoord};

#[derive(Clone, Default, Debug)]
struct Tree {
    height: i8,
}

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}
impl Direction {
    fn get_dx_dy(&self) -> (isize, isize) {
        match self {
            Direction::UP => (0, -1),
            Direction::DOWN => (0, 1),
            Direction::RIGHT => (1, 0),
            Direction::LEFT => (-1, 0),
        }
    }
}

fn build_tree_grid_from_input(input: &str) -> Grid<Tree> {
    let mut input_lines = input.lines().peekable();
    let mut tree_grid: Grid<Tree> = Grid::new(
        input_lines.peek().unwrap().len(),
        input_lines.clone().count(),
    );
    for (y, input_line) in input_lines.enumerate() {
        for (x, raw_height) in input_line.chars().enumerate() {
            let tree = tree_grid
                .get_cell_mut(&GridCoord { x, y })
                .expect("Cell not found!");
            tree.height =
                i8::from_str_radix(&raw_height.to_string(), 10).expect("Couldn't parse height!");
        }
    }
    tree_grid
}

pub fn part_one(input: &str) -> Option<usize> {
    /// Helper function to check if a tree is visible in a certain direction
    fn is_tree_visible_in_direction(
        grid: &Grid<Tree>,
        coord: &GridCoord,
        direction: &Direction,
    ) -> bool {
        let (dx, dy) = direction.get_dx_dy();
        let tree_line = (1..).map_while(|i| {
            let coord = GridCoord {
                x: coord.x.checked_add_signed(dx * i)?,
                y: coord.y.checked_add_signed(dy * i)?,
            };
            Some(grid.get_cell(&coord)?)
        });
        let mut is_visible = true;
        let this_height = grid
            .get_cell(coord)
            .expect("Tree should exist in this cell!")
            .height;
        for tree in tree_line {
            if tree.height >= this_height {
                is_visible = false;
                break;
            }
        }
        is_visible
    }

    // Build tree grid
    let tree_grid: Grid<Tree> = build_tree_grid_from_input(input);

    // Find out how many trees are visible
    let all_directions = [
        Direction::DOWN,
        Direction::UP,
        Direction::RIGHT,
        Direction::LEFT,
    ];
    let num_visible_trees = tree_grid
        .iter_all_cells()
        .filter(|(coord, _)| {
            all_directions
                .iter()
                .any(|direction| is_tree_visible_in_direction(&tree_grid, coord, direction))
        })
        .count();

    Some(num_visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    fn calc_visible_trees_in_direction(
        grid: &Grid<Tree>,
        coord: &GridCoord,
        direction: Direction,
    ) -> u32 {
        let (dx, dy) = direction.get_dx_dy();
        let tree_line = (1..).map_while(|i| {
            let coord = GridCoord {
                x: coord.x.checked_add_signed(dx * i)?,
                y: coord.y.checked_add_signed(dy * i)?,
            };
            Some(grid.get_cell(&coord)?)
        });
        let mut num_visible_trees: u32 = 0;
        let this_height = grid
            .get_cell(coord)
            .expect("Tree should exist in this cell!!")
            .height;
        for tree in tree_line {
            num_visible_trees += 1;
            if tree.height >= this_height {
                break;
            }
        }
        num_visible_trees
    }

    // Build the tree grid
    let tree_grid: Grid<Tree> = build_tree_grid_from_input(input);

    // For each tree, calculate # of visible trees in all directions and calculate the scenic score
    let max_scenic_score = tree_grid
        .iter_all_cells()
        .map(|(coord, _)| {
            let top = calc_visible_trees_in_direction(&tree_grid, coord, Direction::UP);
            let bottom = calc_visible_trees_in_direction(&tree_grid, coord, Direction::DOWN);
            let left = calc_visible_trees_in_direction(&tree_grid, coord, Direction::LEFT);
            let right = calc_visible_trees_in_direction(&tree_grid, coord, Direction::RIGHT);

            top * bottom * left * right
        })
        .max();

    max_scenic_score
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

use advent_of_code::grid::{Grid, GridCoord};

pub fn part_one(input: &str) -> Option<usize> {
    #[derive(Clone, Default, Debug)]
    struct Tree {
        height: i8,
        visible_top: bool,
        visible_left: bool,
        visible_right: bool,
        visible_bottom: bool,
    }

    let mut input_lines = input.lines().peekable();
    let mut tree_grid: Grid<Tree> = Grid::new(
        input_lines.peek().unwrap().len(),
        input_lines.clone().count(),
    );

    for (y, input_line) in input_lines.enumerate() {
        for (x, raw_height) in input_line.chars().enumerate() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            tree.height =
                i8::from_str_radix(&raw_height.to_string(), 10).expect("Couldn't parse height!");
        }
    }

    // check if each tree is visible from left
    for y in 0..tree_grid.height {
        let mut tallest_height: i8 = -1;
        for x in 0..tree_grid.width {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            if tree.height > tallest_height {
                tree.visible_left = true;
                tallest_height = tree.height;
            }
        }
    }

    // check if each tree is visible from right
    for y in 0..tree_grid.height {
        let mut tallest_height: i8 = -1;
        for x in (0..tree_grid.width).rev() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            if tree.height > tallest_height {
                tree.visible_right = true;
                tallest_height = tree.height;
            }
        }
    }

    // check if each tree is visible from top
    for x in 0..tree_grid.width {
        let mut tallest_height: i8 = -1;
        for y in 0..tree_grid.height {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            if tree.height > tallest_height {
                tree.visible_top = true;
                tallest_height = tree.height;
            }
        }
    }

    // check if each tree is visible from bottom
    for x in 0..tree_grid.width {
        let mut tallest_height: i8 = -1;
        for y in (0..tree_grid.height).rev() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            if tree.height > tallest_height {
                tree.visible_bottom = true;
                tallest_height = tree.height;
            }
        }
    }

    // Find out how many trees are visible
    let num_visible_trees = tree_grid
        .iter_all_cells()
        .filter(|(_, tree)| {
            tree.visible_bottom || tree.visible_top || tree.visible_left || tree.visible_right
        })
        .count();

    Some(num_visible_trees)
}

pub fn part_two(input: &str) -> Option<u32> {
    #[derive(Clone, Default, Debug)]
    struct Tree {
        height: i8,
        num_visible_top: u32,
        num_visible_left: u32,
        num_visible_right: u32,
        num_visible_bottom: u32,
    }
    impl Tree {
        fn calc_scenic_score(&self) -> u32 {
            self.num_visible_bottom
                * self.num_visible_top
                * self.num_visible_left
                * self.num_visible_right
        }
    }

    let mut input_lines = input.lines().peekable();
    let mut tree_grid: Grid<Tree> = Grid::new(
        input_lines.peek().unwrap().len(),
        input_lines.clone().count(),
    );

    for (y, input_line) in input_lines.enumerate() {
        for (x, raw_height) in input_line.chars().enumerate() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            tree.height =
                i8::from_str_radix(&raw_height.to_string(), 10).expect("Couldn't parse height!");
        }
    }

    // for each tree, check how many trees are visible from left
    for y in 0..tree_grid.height {
        let mut prev_tree_heights: Vec<i8> = vec![];
        for x in 0..tree_grid.width {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            let mut num_visible: u32 = 0;
            for prev_height in prev_tree_heights.iter().rev() {
                num_visible += 1;
                if *prev_height >= tree.height {
                    break;
                }
            }
            tree.num_visible_left = num_visible;
            prev_tree_heights.push(tree.height);
        }
    }

    // for each tree, check how many trees are visible from right
    for y in 0..tree_grid.height {
        let mut prev_tree_heights: Vec<i8> = vec![];
        for x in (0..tree_grid.width).rev() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            let mut num_visible: u32 = 0;
            for prev_height in prev_tree_heights.iter().rev() {
                num_visible += 1;
                if *prev_height >= tree.height {
                    break;
                }
            }
            tree.num_visible_right = num_visible;
            prev_tree_heights.push(tree.height);
        }
    }

    // for each tree, check how many trees are visible from top
    for x in 0..tree_grid.width {
        let mut prev_tree_heights: Vec<i8> = vec![];
        for y in 0..tree_grid.height {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            let mut num_visible: u32 = 0;
            for prev_height in prev_tree_heights.iter().rev() {
                num_visible += 1;
                if *prev_height >= tree.height {
                    break;
                }
            }
            tree.num_visible_top = num_visible;
            prev_tree_heights.push(tree.height);
        }
    }

    // for each tree, check how many trees are visible from bottom
    for x in 0..tree_grid.width {
        let mut prev_tree_heights: Vec<i8> = vec![];
        for y in (0..tree_grid.height).rev() {
            let tree = tree_grid
                .get_cell_mut(GridCoord { x, y })
                .expect("Cell not found!");
            let mut num_visible: u32 = 0;
            for prev_height in prev_tree_heights.iter().rev() {
                num_visible += 1;
                if *prev_height >= tree.height {
                    break;
                }
            }
            tree.num_visible_bottom = num_visible;
            prev_tree_heights.push(tree.height);
        }
    }

    // Calculate the scenic score of every tree, and find the largest one
    let max_scenic_score = tree_grid
        .iter_all_cells()
        .map(|(_, tree)| tree.calc_scenic_score())
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

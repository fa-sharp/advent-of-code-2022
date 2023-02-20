// Thanks to fasterthanlime https://fasterthanli.me/series/advent-of-code-2022/part-8

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct GridCoord {
    pub x: usize,
    pub y: usize,
}

impl From<(usize, usize)> for GridCoord {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

/// Implementation of a grid structure. Uses a HashMap as the underlying data structure.
#[derive(Debug)]
pub struct Grid<T> {
    pub width: usize,
    pub height: usize,
    data: HashMap<GridCoord, T>,
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new(width: usize, height: usize) -> Self {
        let mut data: HashMap<GridCoord, T> = HashMap::with_capacity(width * height);
        for x in 0..width {
            for y in 0..height {
                data.insert(GridCoord { x, y }, T::default());
            }
        }
        Self {
            width,
            height,
            data,
        }
    }

    fn is_in_bounds(&self, coord: &GridCoord) -> bool {
        coord.x < self.width && coord.y < self.height
    }

    pub fn get_cell_mut(&mut self, coord: &GridCoord) -> Option<&mut T> {
        if !self.is_in_bounds(coord) {
            None
        } else {
            self.data.get_mut(coord)
        }
    }

    pub fn get_cell(&self, coord: &GridCoord) -> Option<&T> {
        if !self.is_in_bounds(coord) {
            None
        } else {
            self.data.get(coord)
        }
    }

    pub fn iter_all_cells(&self) -> std::collections::hash_map::Iter<GridCoord, T> {
        self.data.iter()
    }
}

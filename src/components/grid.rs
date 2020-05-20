use ggez::nalgebra::{Point2, Vector2};
use specs::prelude::{Component, VecStorage};

// Grid position uses an axial coordinate system so we can make full use of the cube coordinate
// system with only two coordinates (https://www.redblobgames.com/grids/hexagons/#coordinates). You
// can find a bunch of utility functions for this coordinate system in src/utils/grid.rs
#[derive(Debug)]
pub struct GridPosition(pub Point2<i32>);
impl Component for GridPosition {
    type Storage = VecStorage<Self>;
}

impl GridPosition {
    pub fn adjacent_tiles(&self) -> Vec<Point2<i32>> {
        let left_offset = if self.0.y % 2 == 0 { 0 } else { -1 };
        let right_offset = if self.0.y % 2 == 0 { 1 } else { 0 };
        vec![
            Vector2::new(left_offset, -1),
            Vector2::new(right_offset, -1),
            Vector2::new(1, 0),
            Vector2::new(-1, 0),
            Vector2::new(0, 0),
            Vector2::new(left_offset, 1),
            Vector2::new(right_offset, 1),
        ]
        .into_iter()
        .map(|p| self.0 + p)
        .collect()
    }
}

#[derive(Debug)]
pub struct Selectable {
    pub selected: bool,
}
impl Component for Selectable {
    type Storage = VecStorage<Self>;
}

impl Selectable {
    pub fn new() -> Selectable {
        Selectable { selected: true }
    }
}

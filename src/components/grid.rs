use anyhow::Result;
use specs::prelude::{Component, VecStorage};

pub struct GridPosition(pub Point2<i32>);
impl Component for GridPosition {
    type Storage = VecStorage<Self>;
}

impl GridPosition {
    pub fn adjacent_tiles(&self) -> Vec<Point2<i32>> {
        let left_offset = if self.0.y % 2 == 0 { 0 } else { -1 };
        let right_offset = if self.0.y % 2 == 0 { 1 } else { 0 };
        vec![
            Point2::new(left_offset, -1),
            Point2::new(right_offset, -1),
            Point2::new(1, 0),
            Point2::new(-1, 0),
            Point2::new(),
            Point2::new(left_offset, 1),
            Point2::new(right_offset, 1),
        ]
        .into_iter()
        .map(|p| p + self.0)
        .collect()
    }
}

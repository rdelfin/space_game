use crate::utils::grid;

use anyhow::Result;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Component, NullStorage, VecStorage};
use std::cmp::Ordering;
use std::collections::HashSet;

// Grid position uses an axial coordinate system so we can make full use of the cube coordinate
// system with only two coordinates (https://www.redblobgames.com/grids/hexagons/#coordinates). You
// can find a bunch of utility functions for this coordinate system in src/utils/grid.rs
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct GridPosition(pub Point2<i32>);

impl GridPosition {
    #[allow(dead_code)]
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

#[derive(Component, Debug, Default)]
#[storage(NullStorage)]
pub struct Placing;

pub struct Wall {
    s: Point2<i32>,
    e: Point2<i32>,
}

impl Wall {
    pub fn new(start: Point2<i32>, end: Point2<i32>) -> Wall {
        match Wall::pos_ordering(start, end) {
            Ordering::Greater => Wall { s: start, e: end },
            _ => Wall { s: end, e: start },
        }
    }

    fn pos_ordering(a: Point2<i32>, b: Point2<i32>) -> Ordering {
        if a == b {
            Ordering::Equal
        } else if a.x > b.x || a.y > b.y {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

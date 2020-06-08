use crate::utils::people::PathFindingAlgorithm;

use ggez::nalgebra::Point2;
use specs::{Component, VecStorage};
use std::time::Duration;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct PathFinder {
    pub goal: Point2<i32>,
    pub algorithm: PathFindingAlgorithm,
    pub time_in_move: Duration,
    pub move_time: Duration,
}

impl PathFinder {
    pub fn new(
        goal: Point2<i32>,
        algorithm: PathFindingAlgorithm,
        move_time: Duration,
    ) -> PathFinder {
        PathFinder {
            goal,
            algorithm,
            time_in_move: Duration::new(0, 0),
            move_time,
        }
    }

    pub fn set_goal(&mut self, goal: Point2<i32>) {
        self.goal = goal;
        self.time_in_move = Duration::new(0, 0);
    }

    pub fn step(&mut self, delta: &Duration) -> bool {
        self.time_in_move += *delta;
        if self.time_in_move >= self.move_time {
            self.time_in_move = Duration::new(0, 0);
            true
        } else {
            false
        }
    }
}

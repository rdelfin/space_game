use ggez::nalgebra::Point2;

use crate::utils::datastructs::RevRange;

#[derive(Debug, Copy, Clone)]
pub enum PathFindingAlgorithm {
    Manhattan,
}

pub fn get_path(
    start: Point2<i32>,
    end: Point2<i32>,
    algo: PathFindingAlgorithm,
) -> Vec<Point2<i32>> {
    match algo {
        PathFindingAlgorithm::Manhattan => manhattan(start, end),
    }
}

// This algorithm will attempt to match the coordinates of one axis, followed by the other, in a
// way akin to how you might walk from one place to another in a grided system of streets, going
// north or south along one road, until you reach an east-west road that matches the destination
// you're going to.
fn manhattan(start: Point2<i32>, end: Point2<i32>) -> Vec<Point2<i32>> {
    // Picking x coordinate first
    let mut res = vec![];

    let x_range = RevRange::new(start.x, end.x);
    let y_range = RevRange::new(start.y, end.y);
    let mut curr_pos = start;

    for x in x_range {
        curr_pos.x = x;
        res.push(curr_pos);
    }

    for y in y_range {
        curr_pos.y = y;
        res.push(curr_pos);
    }

    res
}

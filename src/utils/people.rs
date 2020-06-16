use crate::components::Wall;
use crate::utils::datastructs::RevRange;
use crate::utils::grid;

use ggez::nalgebra::Point2;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
pub enum PathFindingAlgorithm {
    Manhattan,
    BreathFirst,
}

pub fn get_path(
    start: Point2<i32>,
    end: Point2<i32>,
    walls: &HashSet<&Wall>,
    algo: PathFindingAlgorithm,
) -> Vec<Point2<i32>> {
    match algo {
        PathFindingAlgorithm::Manhattan => manhattan(start, end),
        PathFindingAlgorithm::BreathFirst => breath_first(start, end, walls),
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

// A basic, breath-first search algorithm.
fn breath_first(start: Point2<i32>, end: Point2<i32>, walls: &HashSet<&Wall>) -> Vec<Point2<i32>> {
    let mut frontier: VecDeque<Point2<i32>> = VecDeque::new();
    frontier.push_back(start);
    let mut came_from: HashMap<Point2<i32>, Point2<i32>> = HashMap::new();

    while let Some(p) = frontier.pop_front() {
        // Special case for exiting when we reach our goal
        if p == end {
            break;
        }

        let neighbours = grid::neighbours(p);

        for n in neighbours.iter() {
            if !came_from.contains_key(n) && !walls.contains(&Wall::new(p, *n)) {
                frontier.push_back(*n);
                came_from.insert(*n, p);
            }
        }
    }

    rollback_path(&came_from, start, end)
}

fn rollback_path(
    came_from: &HashMap<Point2<i32>, Point2<i32>>,
    start: Point2<i32>,
    end: Point2<i32>,
) -> Vec<Point2<i32>> {
    match came_from.get(&end) {
        Some(_) => {
            let mut path = vec![];
            let mut curr = end;

            while curr != start {
                path.push(curr);
                curr = came_from[&curr];
            }

            path.iter().rev().copied().collect()
        }
        None => vec![],
    }
}

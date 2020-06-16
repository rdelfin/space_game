use crate::components::{GridPosition, PathFinder, Wall};
use crate::resources::DeltaTime;
use crate::utils::people as putils;

use specs::{Join, Read, ReadStorage, System, WriteStorage};
use std::collections::HashSet;

pub struct PathMovementSystem;

impl<'a> System<'a> for PathMovementSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, PathFinder>,
        WriteStorage<'a, GridPosition>,
        ReadStorage<'a, Wall>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut path_finders, mut grid_positions, walls) = data;
        let time = time.0;

        let wall_set = walls.join().collect::<HashSet<_>>();

        for (path_finder, grid_position) in (&mut path_finders, &mut grid_positions).join() {
            if path_finder.step(&time) {
                let path = putils::get_path(
                    grid_position.0,
                    path_finder.goal,
                    &wall_set,
                    path_finder.algorithm,
                );

                if let Some(pos) = path.get(0) {
                    grid_position.0 = *pos
                }
            }
        }
    }
}

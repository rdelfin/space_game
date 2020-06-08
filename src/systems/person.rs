use crate::components::{GridPosition, PathFinder};
use crate::resources::DeltaTime;
use crate::utils::people as putils;

use specs::{Read, System, WriteStorage};

pub struct PathMovementSystem;

impl<'a> System<'a> for PathMovementSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, PathFinder>,
        WriteStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (time, mut path_finders, mut grid_positions) = data;
        let time = time.0;

        use specs::Join;
        for (path_finder, grid_position) in (&mut path_finders, &mut grid_positions).join() {
            if path_finder.step(&time) {
                let path =
                    putils::get_path(grid_position.0, path_finder.goal, path_finder.algorithm);

                if let Some(pos) = path.get(0) {
                    grid_position.0 = *pos
                }
            }
        }
    }
}

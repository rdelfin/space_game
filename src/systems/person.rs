use crate::components::{GridPosition, PathFinder, Wall};
use crate::resources::{DeltaTime, MouseMode, MouseState};
use crate::utils::{grid as gutils, people as putils};

use ggez::input::mouse::MouseButton;
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

pub struct TargetSetSystem;

impl<'a> System<'a> for TargetSetSystem {
    type SystemData = (Read<'a, MouseState>, WriteStorage<'a, PathFinder>);

    fn run(&mut self, data: Self::SystemData) {
        let (mouse_state, mut path_finders) = data;

        if let MouseMode::Free = mouse_state.mode {
            if mouse_state.just_released(MouseButton::Right) {
                for path_finder in (&mut path_finders).join() {
                    let new_goal = gutils::position_to_grid(mouse_state.position());
                    println!("New goal: {}", new_goal);
                    path_finder.set_goal(new_goal);
                }
            }
        }
    }
}

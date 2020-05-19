use crate::components::{GridPosition, Position};

use ggez::nalgebra::Point2;
use specs::{ReadStorage, System, WriteStorage};

const TILE_WIDTH: i32 = 52;
const TILE_HEIGHT: i32 = 60;

pub struct TilePositionSystem;

impl<'a> System<'a> for TilePositionSystem {
    type SystemData = (ReadStorage<'a, GridPosition>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (grid_positions, mut positions) = data;

        use specs::Join;
        for (grid_position, position) in (&grid_positions, &mut positions).join() {
            let x_offset = if grid_position.0.x % 2 == 0 {
                TILE_WIDTH / 2
            } else {
                0
            };

            position.0 = Point2::new(
                (grid_position.0.x * TILE_WIDTH + x_offset) as f32,
                (grid_position.0.y * TILE_HEIGHT * 2 / 3) as f32,
            );
        }
    }
}

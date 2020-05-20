use crate::components::{GridPosition, Position};
use crate::utils::grid;

use ggez::nalgebra::Vector2;
use specs::{ReadStorage, System, WriteStorage};

pub struct TilePositionSystem;

impl<'a> System<'a> for TilePositionSystem {
    type SystemData = (ReadStorage<'a, GridPosition>, WriteStorage<'a, Position>);

    fn run(&mut self, data: Self::SystemData) {
        let (grid_positions, mut positions) = data;

        use specs::Join;
        for (grid_position, position) in (&grid_positions, &mut positions).join() {
            // Subtract half a tile since the coordinate system gives us tile centers
            position.0 = grid::grid_to_position(grid_position.0)
                - Vector2::new(
                    grid::TILE_WIDTH as f32 / 2.0,
                    grid::TILE_HEIGHT as f32 / 2.0,
                );
        }
    }
}

use crate::components::{GridPosition, Selectable};
use crate::resources::MouseState;
use crate::utils::grid;

use ggez::input::mouse::MouseButton;
use specs::{Read, System, WriteStorage};

pub struct TileDragSystem;

impl<'a> System<'a> for TileDragSystem {
    type SystemData = (
        Read<'a, MouseState>,
        WriteStorage<'a, Selectable>,
        WriteStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mouse_state, mut selectables, mut grid_positions) = data;

        use specs::Join;
        for (selectable, grid_position) in (&mut selectables, &mut grid_positions).join() {
            if selectable.selected {
                grid_position.0 = grid::position_to_grid(mouse_state.position());

                if mouse_state.pressed(MouseButton::Left) {
                    selectable.selected = false;
                }
            }
        }
    }
}

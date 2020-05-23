use crate::components::{GridPosition, Selectable};
use crate::entities::BuildingFactory;
use crate::resources::MouseState;
use crate::utils::grid;

use ggez::input::mouse::MouseButton;
use rand;
use specs::{Entities, LazyUpdate, Read, System, WriteStorage};

pub struct TileDragSystem;

impl<'a> System<'a> for TileDragSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, MouseState>,
        WriteStorage<'a, Selectable>,
        WriteStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mouse_state, mut selectables, mut grid_positions) = data;

        use specs::Join;
        for (selectable, grid_position) in (&mut selectables, &mut grid_positions).join() {
            if selectable.selected {
                grid_position.0 = grid::position_to_grid(mouse_state.position());

                if mouse_state.just_released(MouseButton::Left) {
                    selectable.selected = false;
                    let new_building = entities.create();
                    if rand::random() {
                        BuildingFactory::fill_factory(new_building, &updater, grid_position.0)
                            .unwrap();
                    } else {
                        BuildingFactory::fill_home(new_building, &updater, grid_position.0)
                            .unwrap();
                    }
                }
            }
        }
    }
}

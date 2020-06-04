use crate::components::{
    ButtonActionable, ButtonBuilding, GridPosition, Pressable, Pressed, Selected, Sprite,
};
use crate::entities::BuildingFactory;
use crate::resources::MouseState;
use crate::utils::grid;

use ggez::input::mouse::MouseButton;
use ggez::nalgebra::Point2;
use specs::{Entities, LazyUpdate, Read, ReadStorage, System, WriteStorage};

pub struct TileDragSystem;

impl<'a> System<'a> for TileDragSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, MouseState>,
        ReadStorage<'a, Selected>,
        WriteStorage<'a, GridPosition>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mouse_state, selected, mut grid_positions) = data;

        use specs::Join;
        for (entity, _, grid_position) in (&entities, &selected, &mut grid_positions).join() {
            grid_position.0 = grid::position_to_grid(mouse_state.position());

            if mouse_state.just_released(MouseButton::Left) {
                updater.remove::<Selected>(entity);
            }
        }
    }
}

pub struct ButtonPressSystem;

impl<'a> System<'a> for ButtonPressSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, MouseState>,
        ReadStorage<'a, Pressable>,
        ReadStorage<'a, Pressed>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mouse_state, pressables, pressed_ones) = data;

        if mouse_state.just_pressed(MouseButton::Left) {
            use specs::Join;
            for (entity, pressable, ()) in (&entities, &pressables, !&pressed_ones).join() {
                if pressable.click_box.contains(mouse_state.position()) {
                    updater.insert(entity, Pressed);
                }
            }
        }

        if mouse_state.just_released(MouseButton::Left) {
            use specs::Join;
            for (entity, _, _) in (&entities, &pressables, &pressed_ones).join() {
                updater.remove::<Pressed>(entity);
                updater.insert(entity, ButtonActionable);
            }
        }
    }
}

pub struct ButtonSpriteSystem;

impl<'a> System<'a> for ButtonSpriteSystem {
    type SystemData = (
        ReadStorage<'a, Pressable>,
        ReadStorage<'a, Pressed>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (pressables, pressed_ones, mut sprites) = data;

        use specs::Join;
        for (_, pressed, sprite) in (&pressables, (&pressed_ones).maybe(), &mut sprites).join() {
            sprite.curr_frame = Point2::new(
                match pressed {
                    Some(_) => 1,
                    None => 0,
                },
                0,
            );
        }
    }
}

pub struct TileButtonActionSystem;

impl<'a> System<'a> for TileButtonActionSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, ButtonActionable>,
        ReadStorage<'a, ButtonBuilding>,
    );

    fn run(&mut self, data: Self::SystemData) {
        use specs::Join;
        let (entities, updater, actionables, button_buildings) = data;

        for (entity, _, button_building) in (&entities, &actionables, &button_buildings).join() {
            // Mark the action as already acted on for the button
            updater.remove::<ButtonActionable>(entity);

            // Create a new tile
            let tile = entities.create();
            BuildingFactory::fill_tile(
                tile,
                &updater,
                button_building.building_type,
                Point2::new(0, 0),
            );
        }
    }
}

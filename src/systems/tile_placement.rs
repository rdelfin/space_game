use crate::components::{
    ButtonActionable, ButtonBuilding, GridPosition, Placing, Pressable, Pressed, Sprite, Wall,
};
use crate::entities::BuildingFactory;
use crate::resources::{MouseMode, MouseState};
use crate::utils::{buildings, grid};

use ggez::input::mouse::MouseButton;
use ggez::nalgebra::Point2;
use specs::{Entities, Join, LazyUpdate, Read, ReadStorage, System, Write, WriteStorage};

pub struct TileDragSystem;

impl<'a> System<'a> for TileDragSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Write<'a, MouseState>,
        ReadStorage<'a, Placing>,
        WriteStorage<'a, GridPosition>,
        WriteStorage<'a, Wall>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mut mouse_state, placing, mut grid_positions, mut walls) = data;

        for (entity, _, grid_position) in (&entities, &placing, &mut grid_positions).join() {
            grid_position.0 = grid::position_to_grid(mouse_state.position());

            if let MouseMode::PlacingBuildings = mouse_state.mode {
                if mouse_state.just_released(MouseButton::Left) {
                    mouse_state.mode = MouseMode::Free;
                    updater.remove::<Placing>(entity);
                    // Update the walls. Logic can be a bit complicated so moving this
                    // to a separate method
                    buildings::update_walls(&mut walls, &entities, &updater, grid_position.0);
                }
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
            for (entity, pressable, ()) in (&entities, &pressables, !&pressed_ones).join() {
                if pressable.click_box.contains(mouse_state.position()) {
                    updater.insert(entity, Pressed);
                }
            }
        }

        if mouse_state.just_released(MouseButton::Left) {
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
        Write<'a, MouseState>,
        Read<'a, LazyUpdate>,
        ReadStorage<'a, ButtonActionable>,
        ReadStorage<'a, ButtonBuilding>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut mouse_state, updater, actionables, button_buildings) = data;

        for (entity, _, button_building) in (&entities, &actionables, &button_buildings).join() {
            // Mark the action as already acted on for the button
            updater.remove::<ButtonActionable>(entity);

            if let MouseMode::Free = mouse_state.mode {
                mouse_state.mode = MouseMode::PlacingBuildings;
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
}

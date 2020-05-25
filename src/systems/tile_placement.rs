use crate::components::{GridPosition, Pressable, Pressed, Selected, Sprite};
use crate::entities::BuildingFactory;
use crate::resources::MouseState;
use crate::utils::grid;

use ggez::input::mouse::MouseButton;
use ggez::nalgebra::Point2;
use rand;
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
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, updater, mouse_state, pressables, pressed_ones, mut sprites) = data;

        if mouse_state.just_pressed(MouseButton::Left) {
            use specs::Join;
            for (entity, pressable, (), sprite) in
                (&entities, &pressables, !&pressed_ones, &mut sprites).join()
            {
                if pressable.click_box.contains(mouse_state.position()) {
                    updater.insert(entity, Pressed);
                }
            }
        }

        if mouse_state.just_released(MouseButton::Left) {
            use specs::Join;
            for (entity, pressable, _, sprite) in
                (&entities, &pressables, &pressed_ones, &mut sprites).join()
            {
                updater.remove::<Pressed>(entity);
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

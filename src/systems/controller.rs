use crate::components::{RigidBody, UserControlled};
use crate::resources::KeyboardState;

use ggez::input::keyboard::KeyCode;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        Read<'a, KeyboardState>,
        ReadStorage<'a, UserControlled>,
        WriteStorage<'a, RigidBody>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (key_state, _, mut bodies) = data;

        use specs::Join;
        for body in (&mut bodies).join() {
            let (left_pressed, right_pressed, up_pressed, down_pressed) = (
                key_state.is_pressed(KeyCode::A),
                key_state.is_pressed(KeyCode::D),
                key_state.is_pressed(KeyCode::W),
                key_state.is_pressed(KeyCode::S),
            );

            if left_pressed == right_pressed {
                body.vel.x = 0.0;
            } else if left_pressed {
                body.vel.x = -200.0;
            } else {
                body.vel.x = 200.0;
            }

            if up_pressed == down_pressed {
                body.vel.y = 0.0;
            } else if up_pressed {
                body.vel.y = -200.0;
            } else {
                body.vel.y = 200.0;
            }
        }
    }
}

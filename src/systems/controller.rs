use crate::components::{UserControlled, Velocity};
use crate::resources::KeyboardState;

use ggez::input::keyboard::KeyCode;
use specs::{Read, ReadStorage, System, WriteStorage};

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        Read<'a, KeyboardState>,
        ReadStorage<'a, UserControlled>,
        WriteStorage<'a, Velocity>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (key_state, _, mut velocities) = data;

        use specs::Join;
        for vel in (&mut velocities).join() {
            let (left_pressed, right_pressed, up_pressed, down_pressed) = (
                key_state.is_pressed(KeyCode::A),
                key_state.is_pressed(KeyCode::D),
                key_state.is_pressed(KeyCode::W),
                key_state.is_pressed(KeyCode::S),
            );

            if left_pressed == right_pressed {
                vel.0.x = 0.0;
            } else if left_pressed {
                vel.0.x = -200.0;
            } else {
                vel.0.x = 200.0;
            }

            if up_pressed == down_pressed {
                vel.0.y = 0.0;
            } else if up_pressed {
                vel.0.y = -200.0;
            } else {
                vel.0.y = 200.0;
            }

            println!("VEL: {:?}", vel);
        }
    }
}

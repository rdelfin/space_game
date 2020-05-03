use crate::components::{RigidBody, UserControlled};
use crate::resources::KeyboardState;

use ggez::input::keyboard::KeyCode;
use ggez::nalgebra::Rotation2;
use specs::{Read, System, WriteStorage};

pub struct ControllerSystem;

impl<'a> System<'a> for ControllerSystem {
    type SystemData = (
        Read<'a, KeyboardState>,
        WriteStorage<'a, UserControlled>,
        WriteStorage<'a, RigidBody>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (key_state, mut users, mut bodies) = data;

        use specs::Join;
        for (body, user) in (&mut bodies, &mut users).join() {
            let (left_pressed, right_pressed, up_pressed, down_pressed) = (
                key_state.is_pressed(KeyCode::A),
                key_state.is_pressed(KeyCode::D),
                key_state.is_pressed(KeyCode::W),
                key_state.is_pressed(KeyCode::S),
            );

            let mut target_vel = user.get_dir_vec();

            if up_pressed == down_pressed {
                target_vel *= 0.0;
            } else if down_pressed {
                target_vel *= -1.0;
            }

            if left_pressed == right_pressed {
                // Do nothing (no need to touch target vector, but this will make
                // other statements easier)
            } else if left_pressed {
                target_vel = Rotation2::new(user.max_rot_speed) * target_vel;
            } else {
                target_vel = Rotation2::new(-user.max_rot_speed) * target_vel;
            }

            // Only update angle the object is moving.
            body.vel += user.inertia * (target_vel - body.vel);
            if body.vel.norm() > 0.0 {
                user.angle = body.vel.y.atan2(body.vel.x)
            }
        }
    }
}

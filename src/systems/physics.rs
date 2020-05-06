use crate::components::RigidBody;
use crate::resources::DeltaTime;

use specs::{Read, System, WriteStorage};

pub struct PhysicsEngine;

impl<'a> System<'a> for PhysicsEngine {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, RigidBody>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut bodies) = data;
        let delta = delta.0;
        let delta_secs = delta.as_secs_f32();

        use specs::Join;
        for body in (&mut bodies).join() {
            body.pos += body.vel * delta_secs;
            body.vel += body.acc * delta_secs;
        }
    }
}

use crate::components::{Acceleration, Position, Velocity};
use crate::resources::DeltaTime;

use specs::{Read, ReadStorage, System, WriteStorage};

pub struct PhysicsEngine;

impl<'a> System<'a> for PhysicsEngine {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Acceleration>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut positions, mut velocities, accelerations) = data;
        let delta = delta.0;
        let delta_secs = delta.as_secs_f32();

        use specs::Join;
        for (pos, vel, acc) in (&mut positions, &mut velocities, &accelerations).join() {
            pos.0 += vel.0 * delta_secs;
            vel.0 += acc.0 * delta_secs;
        }
    }
}

use crate::components::Sprite;
use crate::resources::DeltaTime;

use specs::{Read, System, WriteStorage};

use std::time::Instant;

pub struct SpriteAnimation;

impl<'a> System<'a> for SpriteAnimation {
    type SystemData = (Read<'a, DeltaTime>, WriteStorage<'a, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut sprites) = data;

        let delta = delta.0;

        use specs::Join;
        for sprite in (&mut sprites).join() {
            sprite.frame_time += delta;
            if sprite.frame_time >= sprite.time_per_frame {
                sprite.frame_time -= sprite.time_per_frame;
                sprite.curr_frame.x += 1;
            }

            if sprite.curr_frame.x >= sprite.sheet_size.x {
                sprite.curr_frame.x = 0;
                sprite.curr_frame.y += 1;
            }

            if sprite.curr_frame.y >= sprite.sheet_size.y {
                sprite.curr_frame.y = 0;
            }
        }
    }
}

use crate::components::{Animated, Sprite};
use crate::resources::DeltaTime;

use specs::{Join, Read, System, WriteStorage};

pub struct SpriteAnimation;

impl<'a> System<'a> for SpriteAnimation {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Animated>,
        WriteStorage<'a, Sprite>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (delta, mut animations, mut sprites) = data;

        let delta = delta.0;
        for (animation, sprite) in (&mut animations, &mut sprites).join() {
            animation.frame_time += delta;
            if animation.frame_time >= animation.time_per_frame {
                animation.frame_time -= animation.time_per_frame;
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

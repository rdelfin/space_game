use crate::components::Sprite;

use specs::{System, WriteStorage};

use std::time::Instant;

pub struct SpriteAnimation;

impl<'a> System<'a> for SpriteAnimation {
    type SystemData = (WriteStorage<'a, Sprite>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut sprites) = data;
        use specs::Join;

        for sprite in (&mut sprites).join() {
            let now = Instant::now();
            match sprite.last_frame_time {
                None => {
                    sprite.last_frame_time = Some(now);
                }
                Some(t) => {
                    sprite.frame_time += (now - t);
                    sprite.last_frame_time = Some(now);
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
    }
}

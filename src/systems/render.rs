use crate::components::{RigidBody, Sprite};

use ggez::graphics::{self, DrawParam, Rect};
use ggez::nalgebra::Vector2;
use ggez::Context;
use specs::{ReadStorage, System};

pub struct RenderSystem<'a> {
    ctx: &'a mut Context,
}

impl<'a> RenderSystem<'a> {
    pub fn new(ctx: &mut Context) -> RenderSystem {
        RenderSystem { ctx }
    }
}

impl<'a, 'b> System<'b> for RenderSystem<'a> {
    type SystemData = (ReadStorage<'b, Sprite>, ReadStorage<'b, RigidBody>);

    fn run(&mut self, data: Self::SystemData) {
        let (sprites, bodies) = data;

        use specs::Join;
        for (sprite, body) in (&sprites, &bodies).join() {
            let frame_portion = Vector2::new(1.0, 1.0).component_div(&Vector2::new(
                sprite.sheet_size.x as f32,
                sprite.sheet_size.y as f32,
            ));
            let src = Rect::new(
                (sprite.curr_frame.x as f32) * frame_portion.x,
                (sprite.curr_frame.y as f32) * frame_portion.y,
                frame_portion.x,
                frame_portion.y,
            );
            graphics::draw(
                self.ctx,
                &sprite.spritesheet,
                DrawParam::new().src(src).dest(body.pos).scale(sprite.scale),
            )
            .unwrap();
        }
    }
}

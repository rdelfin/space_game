use crate::components::{Position, Sprite};
use crate::resources::ImageCache;

use ggez::graphics::{self, DrawParam, Rect};
use ggez::nalgebra::Vector2;
use ggez::Context;
use specs::{Join, ReadStorage, System, Write};

pub struct RenderSystem<'a> {
    ctx: &'a mut Context,
}

impl<'a> RenderSystem<'a> {
    pub fn new(ctx: &mut Context) -> RenderSystem {
        RenderSystem { ctx }
    }
}

impl<'a, 'b> System<'b> for RenderSystem<'a> {
    type SystemData = (
        Write<'b, ImageCache>,
        ReadStorage<'b, Sprite>,
        ReadStorage<'b, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut img_cache, sprites, positions) = data;
        for (sprite, position) in (&sprites, &positions).join() {
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
            let image = img_cache.fetch_image(self.ctx, &sprite.path).unwrap();

            graphics::draw(
                self.ctx,
                image,
                DrawParam::new()
                    .src(src)
                    .dest(position.0)
                    .scale(sprite.scale),
            )
            .unwrap();
        }
    }
}

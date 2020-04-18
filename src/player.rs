use anyhow;
use ggez::graphics::{self, DrawParam, Image};
use ggez::timer;
use ggez::Context;
use nalgebra::Point2;
use std::time::Duration;

pub struct Player {
    frames: Vec<Image>,
    curr_frame: usize,
    frame_delta: Duration,
    first_frame: bool,
}

impl Player {
    pub fn new(ctx: &mut Context) -> anyhow::Result<Player> {
        let mut frames = vec![];
        for i in 1..16 {
            frames.push(Image::new(ctx, format!("/walk{}.png", i))?);
        }

        Ok(Player {
            frames,
            curr_frame: 0,
            frame_delta: Duration::new(0, 0),
            first_frame: true,
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        if self.first_frame {
            self.first_frame = false;
        } else {
            self.frame_delta += timer::delta(ctx);
        }

        if self.frame_delta > Duration::new(0, 41000000) {
            self.curr_frame += 1;
            if self.curr_frame >= self.frames.len() {
                self.curr_frame = 0;
            }
            self.frame_delta -= Duration::new(0, 41000000);
        }
        Ok(())
    }

    pub fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::draw(ctx, &self.frames[self.curr_frame], DrawParam::new())?;
        Ok(())
    }
}

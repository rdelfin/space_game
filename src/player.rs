use anyhow;
use ggez::graphics::{self, DrawParam, Image};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;
use ggez::Context;
use std::time::Duration;

pub struct Player {
    frames: Vec<Image>,
    curr_frame: usize,
    frame_delta: Duration,
    first_frame: bool,
    velocity: Vector2<f32>,
    position: Point2<f32>,
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
            velocity: Vector2::new(0.0, 0.0),
            position: Point2::new(0.0, 0.0),
        })
    }

    pub fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        if self.first_frame {
            self.first_frame = false;
            return Ok(());
        }

        self.frame_delta += timer::delta(ctx);

        if self.frame_delta > Duration::new(0, 41000000) {
            self.curr_frame += 1;
            if self.curr_frame >= self.frames.len() {
                self.curr_frame = 0;
            }
            self.frame_delta -= Duration::new(0, 41000000);
        }

        let left_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::A);
        let right_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::D);
        let up_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::W);
        let down_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::S);

        self.velocity.x = self.calc_direction_velocity(500.0, right_pressed, left_pressed);
        self.velocity.y = self.calc_direction_velocity(500.0, down_pressed, up_pressed);

        self.position += self.velocity * (timer::delta(ctx).as_millis() as f32) / 1000.0;

        Ok(())
    }

    fn calc_direction_velocity(&self, max_vel: f32, positive: bool, negative: bool) -> f32 {
        if positive == negative {
            return 0.0;
        } else if positive {
            return max_vel;
        } else {
            return -max_vel;
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::draw(ctx, &self.frames[self.curr_frame], (self.position,))?;
        Ok(())
    }
}

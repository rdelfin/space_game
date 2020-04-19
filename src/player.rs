use anyhow;
use ggez::graphics::{self, DrawParam, Image};
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::timer;
use ggez::Context;
use std::collections::HashMap;
use std::fmt::Display;
use std::time::Duration;

#[derive(Display, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AnimationState {
    Walking,
    Running,
    Static,
}

pub struct Player {
    frames: HashMap<AnimationState, Vec<Image>>,
    curr_frame: usize,
    frame_delta: Duration,
    first_frame: bool,
    velocity: Vector2<f32>,
    position: Point2<f32>,
    astate: AnimationState,
    flipped: bool,
}

impl Player {
    pub fn new(ctx: &mut Context) -> anyhow::Result<Player> {
        let mut frames = HashMap::new();

        for state in vec![
            AnimationState::Walking,
            AnimationState::Running,
            AnimationState::Static,
        ] {
            frames.insert(state.clone(), vec![]);
            let prefix = Player::astate_to_prefix(&state);
            for i in 1..16 {
                frames
                    .get_mut(&state)
                    .ok_or(anyhow::anyhow!(
                        "frame HashMap key for state {} does not exist, even if it was just added",
                        state
                    ))?
                    .push(Image::new(ctx, format!("/{}{}.png", prefix, i))?);
            }
        }

        Ok(Player {
            frames,
            curr_frame: 0,
            frame_delta: Duration::new(0, 0),
            first_frame: true,
            velocity: Vector2::new(0.0, 0.0),
            position: Point2::new(0.0, 0.0),
            astate: AnimationState::Static,
            flipped: false,
        })
    }

    fn astate_to_prefix(state: &AnimationState) -> &'static str {
        match state {
            AnimationState::Walking => "walk",
            AnimationState::Static => "idle",
            AnimationState::Running => "run",
        }
    }

    pub fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        if self.first_frame {
            self.first_frame = false;
            return Ok(());
        }

        self.frame_delta += timer::delta(ctx);

        if self.frame_delta > Duration::new(0, 60000000) {
            self.curr_frame += 1;
            self.frame_delta -= Duration::new(0, 60000000);
        }

        let left_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::A);
        let right_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::D);
        let up_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::W);
        let down_pressed = keyboard::is_key_pressed(ctx, keyboard::KeyCode::S);

        self.velocity.x = self.calc_direction_velocity(500.0, right_pressed, left_pressed);
        self.velocity.y = self.calc_direction_velocity(500.0, down_pressed, up_pressed);

        self.astate = if self.velocity.norm() > 0.0 {
            AnimationState::Walking
        } else {
            AnimationState::Static
        };

        if self.velocity.norm() > 0.0 {
            if self.velocity.x > 0.0 {
                self.flipped = false;
            } else if self.velocity.x < 0.0 {
                self.flipped = true;
            }
        }

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
        let num_frames = self.frames[&self.astate].len();
        let scale_multiplier = if self.flipped { -1.0 } else { 1.0 };
        let offset = if self.flipped {
            Vector2::new(300.0, 0.0)
        } else {
            Vector2::new(0.0, 0.0)
        };
        graphics::draw(
            ctx,
            &self.frames[&self.astate][self.curr_frame % num_frames],
            DrawParam::new()
                .dest(self.position + offset)
                .scale(Vector2::new(1.0 * scale_multiplier, 1.0)),
        )?;
        Ok(())
    }
}

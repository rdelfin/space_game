use anyhow::Result;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Component, VecStorage};

use std::time::Duration;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position(pub Point2<f32>);

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub path: String,
    pub sheet_size: Point2<usize>,
    pub curr_frame: Point2<usize>,
    pub frame_time: Duration,
    pub time_per_frame: Duration,
    pub scale: Vector2<f32>,
}

impl Sprite {
    pub fn new(
        img_path: &str,
        sheet_size: Point2<usize>,
        time_per_frame: Duration,
        scale: Vector2<f32>,
    ) -> Result<Sprite> {
        Ok(Sprite {
            path: img_path.to_string(),
            sheet_size,
            curr_frame: Point2::new(0, 0),
            frame_time: Duration::new(0, 0),
            time_per_frame,
            scale,
        })
    }
}

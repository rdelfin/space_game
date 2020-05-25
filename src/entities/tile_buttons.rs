use crate::components::{Position, Pressable, Sprite};

use anyhow::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, World, WorldExt};
use std::time::Duration;

pub struct TileButtonFactory;

impl TileButtonFactory {
    pub fn new_home_button(world: &mut World, position: Point2<f32>) -> Result<Entity> {
        TileButtonFactory::new_button(world, position, "/home_button.png")
    }

    pub fn new_factory_button(world: &mut World, position: Point2<f32>) -> Result<Entity> {
        TileButtonFactory::new_button(world, position, "/factory_button.png")
    }

    fn new_button(world: &mut World, position: Point2<f32>, img_path: &str) -> Result<Entity> {
        Ok(world
            .create_entity()
            .with(Position(position))
            .with(Sprite::new(
                img_path,
                Point2::new(2, 1),
                Vector2::new(1.0, 1.0),
            )?)
            .with(Pressable::new(Rect::new(
                position.x, position.y, 100.0, 100.0,
            )))
            .build())
    }
}

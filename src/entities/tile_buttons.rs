use crate::components::{Position, Pressable, Sprite};
use crate::utils::buildings::{self, BuildingType};

use anyhow::Result;
use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, World, WorldExt};
use std::time::Duration;

pub struct TileButtonFactory;

impl TileButtonFactory {
    pub fn new_button(
        world: &mut World,
        building_type: BuildingType,
        position: Point2<f32>,
    ) -> Result<Entity> {
        Ok(world
            .create_entity()
            .with(Position(position))
            .with(Sprite::new(
                buildings::building_button_sprite_path(building_type),
                Point2::new(2, 1),
                Vector2::new(1.0, 1.0),
            )?)
            .with(Pressable::new(Rect::new(
                position.x, position.y, 100.0, 100.0,
            )))
            .build())
    }
}

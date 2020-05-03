use crate::components::{RigidBody, Sprite};

use anyhow::Result;
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use specs::Entity;
use specs::{Builder, World, WorldExt};
use std::time::Duration;

pub struct UnitFactory;

impl UnitFactory {
    pub fn new_triangle(
        ctx: &mut Context,
        world: &mut World,
        start_pos: Point2<f32>,
    ) -> Result<Entity> {
        Ok(world
            .create_entity()
            .with(RigidBody::new(start_pos, 0.0))
            .with(Sprite::new(
                ctx,
                "/triangle.png",
                Point2::new(1, 1),
                Duration::new(1, 0),
                Vector2::new(20.0, 20.0),
            )?)
            .build())
    }
}
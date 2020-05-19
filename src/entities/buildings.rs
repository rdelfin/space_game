use crate::components::{GridPosition, Position, Sprite};

use anyhow::Result;
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use specs::Entity;
use specs::{Builder, World, WorldExt};
use std::time::Duration;

pub struct BuildingFactory;

impl BuildingFactory {
    pub fn new_home(
        ctx: &mut Context,
        world: &mut World,
        start_grid: Point2<i32>,
    ) -> Result<Entity> {
        BuildingFactory::new_tile(ctx, world, start_grid, "/home_tile.png")
    }

    pub fn new_factory(
        ctx: &mut Context,
        world: &mut World,
        start_grid: Point2<i32>,
    ) -> Result<Entity> {
        BuildingFactory::new_tile(ctx, world, start_grid, "/factory_tile.png")
    }

    fn new_tile(
        ctx: &mut Context,
        world: &mut World,
        start_grid: Point2<i32>,
        path: &str,
    ) -> Result<Entity> {
        Ok(world
            .create_entity()
            .with(GridPosition(start_grid))
            .with(Position(Point2::new(0.0, 0.0))) // This will get updated accordingly
            .with(Sprite::new(
                ctx,
                path,
                Point2::new(1, 1),
                Duration::new(1, 0),
                Vector2::new(0.4, 0.4),
            )?)
            .build())
    }
}

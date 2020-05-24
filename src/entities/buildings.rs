use crate::components::{Animated, GridPosition, Position, Selected, Sprite};

use anyhow::Result;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, LazyUpdate, World, WorldExt};
use std::time::Duration;

pub struct BuildingFactory;

impl BuildingFactory {
    pub fn new_home(world: &mut World, start_grid: Point2<i32>) -> Result<Entity> {
        BuildingFactory::new_tile(world, start_grid, "/home_tile.png")
    }

    pub fn fill_home(entity: Entity, updater: &LazyUpdate, start_grid: Point2<i32>) -> Result<()> {
        BuildingFactory::fill_tile(entity, updater, start_grid, "/home_tile.png")
    }

    pub fn new_factory(world: &mut World, start_grid: Point2<i32>) -> Result<Entity> {
        BuildingFactory::new_tile(world, start_grid, "/factory_tile.png")
    }

    pub fn fill_factory(
        entity: Entity,
        updater: &LazyUpdate,
        start_grid: Point2<i32>,
    ) -> Result<()> {
        BuildingFactory::fill_tile(entity, updater, start_grid, "/factory_tile.png")
    }

    fn fill_tile(
        entity: Entity,
        updater: &LazyUpdate,
        start_grid: Point2<i32>,
        path: &str,
    ) -> Result<()> {
        updater.insert(entity, GridPosition(start_grid));
        updater.insert(entity, Position(Point2::new(0.0, 0.0)));
        updater.insert(entity, Selected);
        updater.insert(
            entity,
            Sprite::new(path, Point2::new(1, 1), Vector2::new(0.4, 0.4))?,
        );
        Ok(())
    }

    fn new_tile(world: &mut World, start_grid: Point2<i32>, path: &str) -> Result<Entity> {
        Ok(world
            .create_entity()
            .with(GridPosition(start_grid))
            .with(Position(Point2::new(0.0, 0.0))) // This will get updated accordingly
            .with(Selected)
            .with(Sprite::new(
                path,
                Point2::new(1, 1),
                Vector2::new(0.4, 0.4),
            )?)
            .build())
    }
}

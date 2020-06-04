use crate::components::{GridPosition, Position, Selected, Sprite};
use crate::utils::buildings::{self, BuildingType};

use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, LazyUpdate, World, WorldExt};

pub struct BuildingFactory;

impl BuildingFactory {
    #[allow(dead_code)]
    pub fn new_tile(
        world: &mut World,
        building_type: BuildingType,
        start_grid: Point2<i32>,
    ) -> Entity {
        world
            .create_entity()
            .with(GridPosition(start_grid))
            .with(Position(Point2::new(0.0, 0.0))) // This will get updated accordingly
            .with(Selected)
            .with(Sprite::new(
                buildings::building_sprite_path(building_type),
                Point2::new(1, 1),
                Vector2::new(0.4, 0.4),
            ))
            .build()
    }

    pub fn fill_tile(
        entity: Entity,
        updater: &LazyUpdate,
        building_type: BuildingType,
        start_grid: Point2<i32>,
    ) {
        updater.insert(entity, GridPosition(start_grid));
        updater.insert(entity, Position(Point2::new(0.0, 0.0)));
        updater.insert(entity, Selected);
        updater.insert(
            entity,
            Sprite::new(
                buildings::building_sprite_path(building_type),
                Point2::new(1, 1),
                Vector2::new(0.4, 0.4),
            ),
        );
    }
}

use crate::components::{ButtonBuilding, Position, Pressable, Sprite};
use crate::utils::buildings::{self, BuildingType};

use ggez::graphics::Rect;
use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, World, WorldExt};

pub struct TileButtonFactory;

impl TileButtonFactory {
    pub fn new_button(
        world: &mut World,
        building_type: BuildingType,
        position: Point2<f32>,
    ) -> Entity {
        world
            .create_entity()
            .with(Position(position))
            .with(Sprite::new(
                buildings::building_button_sprite_path(building_type),
                Point2::new(2, 1),
                Vector2::new(1.0, 1.0),
            ))
            .with(ButtonBuilding::new(building_type))
            .with(Pressable::new(Rect::new(
                position.x, position.y, 100.0, 100.0,
            )))
            .build()
    }
}

use crate::components::{GridPosition, Position, Sprite};

use ggez::nalgebra::{Point2, Vector2};
use specs::{Builder, Entity, World, WorldExt};

pub struct PeopleFactory;

impl PeopleFactory {
    pub fn new_person(world: &mut World, start_grid: Point2<i32>) -> Entity {
        world
            .create_entity()
            .with(GridPosition(start_grid))
            .with(Position(Point2::new(0.0, 0.0)))
            .with(Sprite::new(
                "/person.png",
                Point2::new(1, 1),
                Vector2::new(0.4, 0.4),
            ))
            .build()
    }
}

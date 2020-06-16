use crate::components::Wall;
use crate::entities::BuildingFactory;
use crate::utils::grid;

use ggez::nalgebra::Point2;
use specs::{Entities, Join, LazyUpdate, WriteStorage};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
pub enum BuildingType {
    Home,
    Factory,
    Airlock,
}

pub fn building_sprite_path(building_type: BuildingType) -> &'static str {
    match building_type {
        BuildingType::Home => "/home_tile.png",
        BuildingType::Factory => "/factory_tile.png",
        BuildingType::Airlock => "/airlock_tile.png",
    }
}

pub fn building_button_sprite_path(building_type: BuildingType) -> &'static str {
    match building_type {
        BuildingType::Home => "/home_button.png",
        BuildingType::Factory => "/factory_button.png",
        BuildingType::Airlock => "/airlock_button.png",
    }
}

pub fn update_walls<'a>(
    walls: &mut WriteStorage<'a, Wall>,
    entities: &Entities<'a>,
    updater: &LazyUpdate,
    position: Point2<i32>,
) {
    let wall_set = walls.join().collect::<HashSet<_>>();

    for neighbour in &grid::neighbours(position) {
        if !wall_set.contains(&Wall::new(position, *neighbour)) {
            let wall = entities.create();
            BuildingFactory::fill_wall(wall, updater, position, *neighbour);
        }
    }
}

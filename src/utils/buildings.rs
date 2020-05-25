#[derive(Debug)]
pub enum BuildingType {
    Home,
    Factory,
}

pub fn building_sprite_path(building_type: BuildingType) -> &'static str {
    match building_type {
        BuildingType::Home => "/home_tile.png",
        BuildingType::Factory => "/factory_tile.png",
    }
}

pub fn building_button_sprite_path(building_type: BuildingType) -> &'static str {
    match building_type {
        BuildingType::Home => "/home_button.png",
        BuildingType::Factory => "/factory_button.png",
    }
}

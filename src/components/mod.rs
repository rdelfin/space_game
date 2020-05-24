use specs::{World, WorldExt};

mod grid;
mod sprite;

pub use self::grid::{GridPosition, Selected};
pub use self::sprite::{Position, Sprite};

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Sprite>();
    world.register::<GridPosition>();
    world.register::<Selected>();
}

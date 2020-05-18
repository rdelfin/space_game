use specs::{World, WorldExt};

mod sprite;
mod user;

pub use self::sprite::{Position, Sprite};
pub use self::user::UserControlled;

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<UserControlled>();
    world.register::<Sprite>();
}

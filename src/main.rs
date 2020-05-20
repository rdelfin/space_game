mod components;
mod entities;
mod resources;
mod systems;
mod utils;

use anyhow;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, FilterMode};
use ggez::input::mouse;
use ggez::nalgebra::Point2;
use ggez::{Context, ContextBuilder};
use specs::prelude::{DispatcherBuilder, World, WorldExt};
use specs::RunNow;

use std::env;
use std::path;

fn main() {
    let resource_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("assets");
            path
        }
        Err(_) => path::PathBuf::from("./assets"),
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Ricardo Delfin")
        .add_resource_path(resource_dir)
        .window_setup(WindowSetup::default().title("Game Test (rdelfin)"))
        .window_mode(WindowMode::default().dimensions(2000.0, 2000.0))
        .build()
        .expect("Could not create ggez context");

    let mut my_game = MyGame::new(&mut ctx).unwrap();

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

struct MyGame {
    world: World,
    first_frame: bool,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> anyhow::Result<MyGame> {
        graphics::set_default_filter(ctx, FilterMode::Nearest);
        mouse::set_cursor_grabbed(ctx, true);

        let mut world = World::new();
        components::register_components(&mut world);

        entities::BuildingFactory::new_home(ctx, &mut world, Point2::new(0, 0))?;

        world.insert(resources::DeltaTime::default());
        world.insert(resources::KeyboardState::default());
        world.insert(resources::MouseState::default());

        Ok(MyGame {
            world,
            first_frame: true,
        })
    }

    fn update_resources(&mut self, ctx: &mut Context) {
        // First frame will include the loading times for images and resources, so we want to skip
        // it when getting time.
        if self.first_frame {
            self.first_frame = false;
        } else {
            let mut delta = self.world.write_resource::<resources::DeltaTime>();
            delta.update(ctx);
        }

        let (mut key_state, mut mouse_state) = (
            self.world.write_resource::<resources::KeyboardState>(),
            self.world.write_resource::<resources::MouseState>(),
        );
        key_state.update(ctx);
        mouse_state.update(ctx);
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        self.update_resources(ctx);

        let mut dispatcher = DispatcherBuilder::new()
            .with(systems::SpriteAnimation, "sprite_animation", &[])
            .with(systems::TileDragSystem, "tile_drag", &[])
            .with(systems::TilePositionSystem, "tile_position", &["tile_drag"])
            .build();
        dispatcher.dispatch(&mut self.world);
        self.world.maintain();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        {
            let mut renderer = systems::RenderSystem::new(ctx);
            renderer.run_now(&self.world);
            self.world.maintain();
        }
        Ok(graphics::present(ctx)?)
    }
}

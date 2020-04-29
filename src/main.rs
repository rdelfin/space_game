#[macro_use]
extern crate enum_display_derive;
#[macro_use]
extern crate specs;

mod components;
mod player;
mod resources;
mod systems;

use anyhow;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Point2;
use ggez::{graphics, timer, Context, ContextBuilder};
use specs::prelude::{Builder, Dispatcher, DispatcherBuilder, Entity, World, WorldExt};
use specs::shred::FetchMut;
use specs::RunNow;

use std::env;
use std::path;
use std::time::Duration;

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
        let mut world = World::new();
        world.register::<components::Position>();
        world.register::<components::Velocity>();
        world.register::<components::Sprite>();

        world
            .create_entity()
            .with(components::Position { x: 10.0, y: 3.0 })
            .with(components::Velocity { dx: 0.0, dy: 0.0 })
            .with(components::Sprite::new(
                ctx,
                "/idle.png",
                Point2::new(15, 1),
                Duration::from_millis(40),
            )?)
            .build();

        world.insert(resources::DeltaTime::default());

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
            (*delta).update(ctx);
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        self.update_resources(ctx);

        let mut dispatcher = DispatcherBuilder::new()
            .with(systems::SpriteAnimation, "sprite_animation", &[])
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

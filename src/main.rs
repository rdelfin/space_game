#[macro_use]
extern crate enum_display_derive;
#[macro_use]
extern crate specs;

mod components;
mod player;

use anyhow;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Vector2;
use ggez::{graphics, Context, ContextBuilder};
use specs::prelude::{Builder, Entity, World, WorldExt};

use std::env;
use std::path;

fn main() {
    let resource_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("assets");
            path
        }
        Err(_) => path::PathBuf::from("./resources"),
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
    entities: Vec<Entity>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> anyhow::Result<MyGame> {
        let mut game = MyGame {
            world: World::new(),
            entities: vec![],
        };

        game.world.register::<components::Position>();
        game.world.register::<components::Velocity>();

        game.entities.push(
            game.world
                .create_entity()
                .with(components::Position { x: 10.0, y: 3.0 })
                .build(),
        );

        Ok(game)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        Ok(graphics::present(ctx)?)
    }
}

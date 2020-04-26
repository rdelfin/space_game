#[macro_use]
extern crate ecs;
#[macro_use]
extern crate enum_display_derive;

mod components;
mod player;
mod systems;

use anyhow;
use ecs::{BuildData, World};
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::nalgebra::Vector2;
use ggez::{graphics, Context, ContextBuilder};

use std::env;
use std::path;

use components::position::{Position, Velocity};
use components::MyComponents;
use player::Player;
use systems::motion::MotionProcess;
use systems::MySystems;

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
    world: World<MySystems>,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> anyhow::Result<MyGame> {
        let mut game = MyGame {
            world: World::<MySystems>::new(),
        };

        let entity =
            game.world
                .create_entity(|entity: BuildData<MyComponents>, data: &mut MyComponents| {
                    data.position.add(
                        &entity,
                        Position {
                            p: Vector2::new(0.0, 0.0),
                        },
                    );
                    data.velocity.add(
                        &entity,
                        Velocity {
                            v: Vector2::new(0.0, 0.0),
                        },
                    );
                });

        Ok(game)
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        self.world.update();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        Ok(graphics::present(ctx)?)
    }
}

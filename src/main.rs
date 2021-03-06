mod components;
mod entities;
mod resources;
mod systems;
mod utils;

#[macro_use]
extern crate specs_derive;

use utils::buildings::BuildingType;

use anyhow;
use ggez::conf::{WindowMode, WindowSetup};
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, FilterMode};
use ggez::input::mouse;
use ggez::nalgebra::Point2;
use ggez::{Context, ContextBuilder};
use specs::prelude::{Dispatcher, DispatcherBuilder, World, WorldExt};
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
        .window_mode(WindowMode::default().dimensions(1800.0, 1000.0))
        .build()
        .expect("Could not create ggez context");

    let mut my_game = MyGame::new(&mut ctx).unwrap();

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

struct MyGame<'a, 'b> {
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    first_frame: bool,
}

impl<'a, 'b> MyGame<'a, 'b> {
    pub fn new(ctx: &mut Context) -> anyhow::Result<MyGame<'a, 'b>> {
        graphics::set_default_filter(ctx, FilterMode::Nearest);
        mouse::set_cursor_grabbed(ctx, true)?;

        let mut world = World::new();
        let mut dispatcher: Dispatcher<'a, 'b> = DispatcherBuilder::new()
            .with(systems::SpriteAnimation, "sprite_animation", &[])
            .with(systems::ButtonPressSystem, "button_press", &[])
            .with(systems::TargetSetSystem, "target_set", &[])
            .with(
                systems::ButtonSpriteSystem,
                "button_sprite",
                &["button_press"],
            )
            .with(
                systems::TileButtonActionSystem,
                "tile_button_action",
                &["button_press"],
            )
            .with(
                systems::TileDragSystem,
                "tile_drag",
                &["tile_button_action"],
            )
            .with(
                systems::PathMovementSystem,
                "path_movement",
                &["target_set"],
            )
            .with(
                systems::TilePositionSystem,
                "tile_position",
                &["tile_drag", "path_movement"],
            )
            .build();

        dispatcher.setup(&mut world);

        world.insert(resources::DeltaTime::default());
        world.insert(resources::KeyboardState::default());
        world.insert(resources::MouseState::default());
        world.insert(resources::ImageCache::default());

        entities::TileButtonFactory::new_button(
            &mut world,
            BuildingType::Home,
            Point2::new(700.0, 800.0),
        );
        entities::TileButtonFactory::new_button(
            &mut world,
            BuildingType::Factory,
            Point2::new(850.0, 800.0),
        );
        entities::TileButtonFactory::new_button(
            &mut world,
            BuildingType::Airlock,
            Point2::new(1000.0, 800.0),
        );
        entities::PeopleFactory::new_person(&mut world, Point2::new(5, 5), Point2::new(7, 3));

        Ok(MyGame {
            world,
            dispatcher,
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

impl<'a, 'b> EventHandler for MyGame<'a, 'b> {
    fn update(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        self.update_resources(ctx);
        self.dispatcher.dispatch(&self.world);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> anyhow::Result<()> {
        graphics::clear(ctx, graphics::WHITE);
        {
            let mut renderer = systems::RenderSystem::new(ctx);
            renderer.run_now(&self.world);
        }
        self.world.maintain();
        Ok(graphics::present(ctx)?)
    }
}

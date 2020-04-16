use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

use std::env;
use std::path;

fn main() {
    let resource_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => {
            let mut path = path::PathBuf::from(manifest_dir);
            path.push("resources");
            path
        }
        Err(_) => path::PathBuf::from("./resources"),
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Ricardo Delfin")
        .add_resource_path(resource_dir)
        .build()
        .expect("Could not create ggez context");

    let mut my_game = MyGame::new(&mut ctx);

    match event::run(&mut ctx, &mut event_loop, &mut my_game) {
        Ok(_) => println!("Exited cleanly."),
        Err(e) => println!("Error occurred: {}", e),
    }
}

struct MyGame {}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {}
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::WHITE);

        graphics::present(ctx)
    }
}

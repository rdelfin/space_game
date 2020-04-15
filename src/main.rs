use ggez::event::{self, EventHandler};
use ggez::{graphics, Context, ContextBuilder, GameResult};

fn main() {
    let (mut ctx, mut event_loop) = ContextBuilder::new("my_game", "Ricardo Delfin")
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

use ggez::{timer, Context};
use std::time::Duration;

#[derive(Default)]
pub struct DeltaTime(pub Duration);

impl DeltaTime {
    pub fn update(&mut self, ctx: &mut Context) {
        self.0 = timer::delta(ctx);
    }
}

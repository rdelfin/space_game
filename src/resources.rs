use ggez::input::keyboard::{self, KeyCode};
use ggez::{timer, Context};
use std::collections::HashSet;
use std::time::Duration;

#[derive(Default)]
pub struct DeltaTime(pub Duration);

impl DeltaTime {
    pub fn update(&mut self, ctx: &mut Context) {
        self.0 = timer::delta(ctx);
    }
}

#[derive(Default)]
pub struct KeyboardState {
    currently_pressed: HashSet<KeyCode>,
    previously_pressed: HashSet<KeyCode>,
}

impl KeyboardState {
    pub fn update(&mut self, ctx: &mut Context) {
        self.previously_pressed = self.currently_pressed.clone();
        self.currently_pressed = keyboard::pressed_keys(ctx).clone();
    }

    pub fn is_pressed(&self, key: KeyCode) -> bool {
        self.currently_pressed.contains(&key)
    }

    pub fn is_released(&self, key: KeyCode) -> bool {
        !self.is_pressed(key)
    }

    #[allow(dead_code)]
    pub fn just_pressed(&self, key: KeyCode) -> bool {
        self.is_pressed(key) && !self.previously_pressed.contains(&key)
    }

    #[allow(dead_code)]
    pub fn just_released(&self, key: KeyCode) -> bool {
        self.is_released(key) && self.previously_pressed.contains(&key)
    }
}

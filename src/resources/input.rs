use ggez::input::keyboard::{self, KeyCode};
use ggez::input::mouse::{self, MouseButton};
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use std::collections::HashSet;

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

pub enum MouseMode {
    Free,
    PlacingBuildings,
}

pub struct MouseState {
    currently_pressed: HashSet<MouseButton>,
    previously_pressed: HashSet<MouseButton>,
    curr_position: Point2<f32>,
    delta: Vector2<f32>,
    pub mode: MouseMode,
}

impl Default for MouseState {
    fn default() -> Self {
        MouseState {
            currently_pressed: HashSet::new(),
            previously_pressed: HashSet::new(),
            curr_position: Point2::new(0.0, 0.0),
            delta: Vector2::new(0.0, 0.0),
            mode: MouseMode::Free,
        }
    }
}

impl MouseState {
    pub fn update(&mut self, ctx: &mut Context) {
        self.previously_pressed = self.currently_pressed.clone();
        self.currently_pressed = HashSet::new();
        for button in &[MouseButton::Left, MouseButton::Middle, MouseButton::Right] {
            if mouse::button_pressed(ctx, *button) {
                self.currently_pressed.insert(*button);
            }
        }
        self.curr_position = Point2::from(mouse::position(ctx));
        // Delta is a point for some reason. Turn into a vector2
        self.delta = Point2::from(mouse::delta(ctx)) - Point2::new(0.0, 0.0);
    }

    pub fn position(&self) -> Point2<f32> {
        self.curr_position
    }

    #[allow(dead_code)]
    pub fn delta(&self) -> Vector2<f32> {
        self.delta
    }

    pub fn pressed(&self, button: MouseButton) -> bool {
        self.currently_pressed.contains(&button)
    }

    #[allow(dead_code)]
    pub fn released(&self, button: MouseButton) -> bool {
        !self.pressed(button)
    }

    pub fn just_pressed(&self, button: MouseButton) -> bool {
        self.pressed(button) && !self.previously_pressed.contains(&button)
    }

    pub fn just_released(&self, button: MouseButton) -> bool {
        !self.pressed(button) && self.previously_pressed.contains(&button)
    }
}

use specs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub dx: f32,
    pub dy: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

use ggez::nalgebra::Vector2;
use specs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct Position(pub Vector2<f32>);

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity(pub Vector2<f32>);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Acceleration(pub Vector2<f32>);

impl Component for Acceleration {
    type Storage = VecStorage<Self>;
}

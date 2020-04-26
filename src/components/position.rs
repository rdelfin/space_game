use ggez::nalgebra::Vector2;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub p: Vector2<f32>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Velocity {
    pub v: Vector2<f32>,
}

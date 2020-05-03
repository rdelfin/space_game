use ggez::nalgebra::{Point2, Vector2};
use specs::prelude::{Component, VecStorage};

pub struct RigidBody {
    pub pos: Point2<f32>,
    pub vel: Vector2<f32>,
    pub acc: Vector2<f32>,
    pub rot: f32,
    pub rot_speed: f32,
}

impl Component for RigidBody {
    type Storage = VecStorage<Self>;
}

impl RigidBody {
    pub fn new(pos: Point2<f32>, rot: f32) -> RigidBody {
        RigidBody {
            pos,
            vel: Vector2::new(0.0, 0.0),
            acc: Vector2::new(0.0, 0.0),
            rot,
            rot_speed: 0.0,
        }
    }
}

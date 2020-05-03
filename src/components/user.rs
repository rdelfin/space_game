use ggez::nalgebra::Vector2;
use specs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct UserControlled {
    pub max_speed: f32,
    pub max_rot_speed: f32,
    pub inertia: f32,
    pub angle: f32,
}

impl UserControlled {
    pub fn new(max_speed: f32, max_rot_speed: f32, inertia: f32) -> UserControlled {
        UserControlled {
            max_speed,
            max_rot_speed,
            inertia,
            angle: 0.0,
        }
    }

    pub fn get_dir_vec(&self) -> Vector2<f32> {
        Vector2::new(self.angle.cos(), self.angle.sin())
    }
}

impl Component for UserControlled {
    type Storage = VecStorage<Self>;
}

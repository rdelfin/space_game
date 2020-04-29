use specs::prelude::{Component, VecStorage};

#[derive(Debug)]
pub struct UserControlled;

impl Component for UserControlled {
    type Storage = VecStorage<Self>;
}

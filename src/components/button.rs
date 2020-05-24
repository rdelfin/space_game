use specs::prelude::{Component, NullStorage};

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct Pressable;

use crate::utils::buildings::BuildingType;

use ggez::graphics::Rect;
use specs::prelude::{Component, NullStorage, VecStorage};

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Pressable {
    pub click_box: Rect,
}
impl Pressable {
    pub fn new(click_box: Rect) -> Pressable {
        Pressable { click_box }
    }
}

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct Pressed;

#[derive(Default, Component, Debug)]
#[storage(NullStorage)]
pub struct ButtonActionable;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct ButtonBuilding {
    pub building_type: BuildingType,
}

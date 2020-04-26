use crate::components::MyComponents;

use ecs::system::EntityProcess;
use ecs::{DataHelper, EntityIter, System};

pub struct MotionProcess;

impl System for MotionProcess {
    type Components = MyComponents;
    type Services = ();
}

impl EntityProcess for MotionProcess {
    fn process(
        &mut self,
        entities: EntityIter<MyComponents>,
        data: &mut DataHelper<MyComponents, ()>,
    ) {
        for e in entities {
            let mut position = data.position[e];
            let mut velocity = data.velocity[e];
        }
    }
}

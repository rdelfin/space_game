pub mod motion;

use crate::components::MyComponents;
use motion::MotionProcess;

use ecs::system::EntitySystem;

systems! {
    struct MySystems<MyComponents, ()> {
        active: {
            motion: EntitySystem<MotionProcess> = EntitySystem::new(
                MotionProcess,
                aspect!(<MyComponents> all: [position, velocity]),
            ),
        },
        passive: {}
    }
}

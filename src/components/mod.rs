pub mod position;

use position::{Position, Velocity};

components! {
    struct MyComponents {
        #[hot] position: Position,
        #[hot] velocity: Velocity,
    }
}

use ggez::graphics;
use specs::{Builder, World};

use components::{AABB, ControledByInput, Position, Velocity};

pub fn create_player(world: &mut World) {
    world
        .create_entity()
        .with(AABB::new(
                graphics::Vector2::new(16.0, 16.0),
                graphics::Vector2::new(40.0, 400.0),
                graphics::Vector2::new(0.0, 0.0)
        ))
        .with(Position::new(graphics::Vector2::new(40.0, 400.0)))
        .with(Velocity::new(graphics::Vector2::new(0.0, 0.0)))
        .with(ControledByInput)
        .build();
}

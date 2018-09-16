use ggez::graphics;
use specs::{Builder, World};

use components::{AABB, Controlable, Position, Velocity};

pub fn create_static(world: &mut World) {
    world
        .create_entity()
        .with(AABB::new(
                graphics::Vector2::new(100.0, 100.0),
                graphics::Vector2::new(200.0, 200.0),
                graphics::Vector2::new(0.0, 0.0)
        ))
        .with(Position::new(graphics::Vector2::new(200.0, 200.0)))
        .with(Velocity::new(graphics::Vector2::new(0.0, 0.0)))
        .build();
}

pub fn create_controled(world: &mut World) {
    world
        .create_entity()
        .with(AABB::new(
                graphics::Vector2::new(16.0, 16.0),
                graphics::Vector2::new(20.0, 400.0),
                graphics::Vector2::new(0.0, 0.0)
        ))
        .with(Position::new(graphics::Vector2::new(20.0, 400.0)))
        .with(Velocity::new(graphics::Vector2::new(0.0, 0.0)))
        .with(Controlable)
        .build();
}

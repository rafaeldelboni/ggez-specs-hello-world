use ggez::graphics;
use specs::{Builder, World};

use components::{Controlable, Square, Velocity};

pub fn create_static(world: &mut World) {
    world
        .create_entity()
        .with(Square {
            shape: graphics::Point2::new(100.0, 100.0),
            position: graphics::Point2::new(10.0, 10.0)})
        .build();
}

pub fn create_moving(world: &mut World) {
    world
        .create_entity()
        .with(Square {
            shape: graphics::Point2::new(400.0, 100.0),
            position: graphics::Point2::new(20.0, 200.0)})
        .with(Velocity { x: 5., y: 5. })
        .build();
}

pub fn create_controled(world: &mut World) {
    world
        .create_entity()
        .with(Square {
            shape: graphics::Point2::new(100.0, 100.0),
            position: graphics::Point2::new(20.0, 400.0)})
        .with(Velocity { x: 0., y: 0. })
        .with(Controlable)
        .build();
}

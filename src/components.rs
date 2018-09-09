use ggez::graphics;

use specs::{Component, NullStorage, VecStorage, World};

pub fn register_components(world: &mut World) {
    world.register::<Text>();
    world.register::<Velocity>();
    world.register::<Controlable>();
}

#[derive(Debug)]
pub struct Text {
    pub value: graphics::Text,
    pub position: graphics::Point2,
}

impl Component for Text {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Debug, Default)]
pub struct Controlable;

impl Component for Controlable {
    type Storage = NullStorage<Self>;
}


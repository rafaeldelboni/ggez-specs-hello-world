use ggez::graphics;
use ggez::{Context};
use ggez::graphics::{Font};
use specs::{Builder, World};

use components::{Controlable, Text, Velocity};

pub fn create_static(ctx: &mut Context, world: &mut World, font: &Font) {
    world
        .create_entity()
        .with(Text {
            value: graphics::Text::new(ctx, "Static text!", &font).unwrap(),
            position: graphics::Point2::new(10.0, 10.0)})
        .build();
}

pub fn create_moving(ctx: &mut Context, world: &mut World, font: &Font) {
    world
        .create_entity()
        .with(Text {
            value: graphics::Text::new(
                       ctx,
                       "I'm a moving alone text!",
                       &font).unwrap(),
                       position: graphics::Point2::new(20.0, 200.0)})
        .with(Velocity { x: 5., y: 5. })
        .build();
}

pub fn create_controled(ctx: &mut Context, world: &mut World, font: &Font) {
    world
        .create_entity()
        .with(Text {
            value: graphics::Text::new(ctx, "Move-me text!", &font).unwrap(),
            position: graphics::Point2::new(20.0, 400.0)})
        .with(Velocity { x: 0., y: 0. })
        .with(Controlable)
        .build();
}

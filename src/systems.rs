use ggez::event;
use ggez::graphics;
use ggez::{Context};
use specs::{System, WriteStorage, Read, ReadStorage, Join};

use resources::{DeltaTime};
use components::{AABB, Position, Controlable, Velocity};

pub struct MoveSystem;

impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, AABB>
    );

    fn run(&mut self, (delta, vel, mut pos, mut aabb): Self::SystemData) {
        (&vel, &mut pos, &mut aabb).join().for_each(|(vel, pos, aabb)| {
            let delta_velocity = vel.get() * delta.0;
            if delta_velocity.x >= 0. && !aabb.pushes_right_wall {
                pos.x(delta_velocity.x);
            }
            if delta_velocity.x < 0. && !aabb.pushes_left_wall {
                pos.x(delta_velocity.x);
            }
            if delta_velocity.y >= 0. && !aabb.pushes_up_wall {
                pos.y(delta_velocity.y);
            }
            if delta_velocity.y < 0. && !aabb.pushes_down_wall {
                pos.y(delta_velocity.y);
            }
            aabb.set_center(pos.current);
        });
    }
}

pub struct RenderingSystem<'c> {
    ctx: &'c mut Context,
}

impl<'c> RenderingSystem<'c> {
    pub fn new(ctx: &'c mut Context) -> RenderingSystem<'c> {
        RenderingSystem { ctx }
    }
}

impl<'a, 'c> System<'a> for RenderingSystem<'c> {
    type SystemData = ReadStorage<'a, AABB>;

    fn run(&mut self, aabb: Self::SystemData) {
        aabb.join().for_each(|aabb| {
            graphics::rectangle(
                self.ctx,
                graphics::DrawMode::Line(1.0),
                graphics::Rect::new(
                    aabb.center.x,
                    aabb.center.y,
                    aabb.fullsize.x,
                    aabb.fullsize.y
                )
            ).unwrap();
        });
    }
}

pub struct ControlSystem {
    keycode: event::Keycode,
    down_event: bool,
}

impl ControlSystem {
    pub fn new(keycode: event::Keycode, down_event: bool) -> ControlSystem {
        ControlSystem { keycode, down_event }
    }
}

impl<'a> System<'a> for ControlSystem {
    type SystemData = (
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Controlable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut velocities, controlables) = data;
        for (vel, _control) in (&mut velocities, &controlables).join() {
            match self.down_event {
                true =>
                    match self.keycode {
                        event::Keycode::Up => vel.y(-200.0),
                        event::Keycode::Down => vel.y(200.0),
                        event::Keycode::Left => vel.x(-200.0),
                        event::Keycode::Right => vel.x(200.0),
                        _ => {}
                    }
                false =>
                    match self.keycode {
                        event::Keycode::Up => vel.y(0.0),
                        event::Keycode::Down => vel.y(0.0),
                        event::Keycode::Left => vel.x(0.0),
                        event::Keycode::Right => vel.x(0.0),
                        _ => {}
                    }
            }
        }
    }
}


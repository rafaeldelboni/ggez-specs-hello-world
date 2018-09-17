use ggez::graphics;
use ggez::{Context};
use specs::{System, WriteStorage, Read, ReadStorage, Join};

use resources::{DeltaTime, InputControls, Map, TileType};
use components::{AABB, ControledByInput, Position, Velocity};

pub struct MoveSystem;
impl MoveSystem {
    pub fn set_collision(
        vel: &graphics::Vector2,
        pos: &mut Position,
        aabb: &mut AABB,
        map: &Map
    ) {
        aabb.collides_right_wall(&pos, &vel);
        aabb.collides_left_wall(&pos, &vel);
        aabb.collides_up_wall(&pos, &vel);
        aabb.collides_down_wall(pos, &vel, &map);
    }
    pub fn set_position(vel: &graphics::Vector2, pos: &mut Position, aabb: &mut AABB) {
        if vel.x >= 0. && !aabb.pushes_right_wall {
            pos.x(vel.x);
        }
        if vel.x < 0. && !aabb.pushes_left_wall {
            pos.x(vel.x);
        }
        if vel.y < 0. && !aabb.pushes_up_wall {
            pos.y(vel.y);
        }
        if vel.y >= 0. && !aabb.pushes_down_wall {
            pos.y(vel.y);
        }
        aabb.set_center(pos.current);
    }
}
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, Map>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, AABB>
    );

    fn run(&mut self, (map, delta, vel, mut pos, mut aabb): Self::SystemData) {
        (&vel, &mut pos, &mut aabb).join().for_each(|(vel, pos, aabb)| {
            let delta_velocity = vel.get() * delta.0;
            MoveSystem::set_position(&delta_velocity, pos, aabb);
            MoveSystem::set_collision(&delta_velocity, pos, aabb, &map);
        });
    }
}

pub struct ControlableSystem;
impl<'a> System<'a> for ControlableSystem {
    type SystemData = (
        Read<'a, InputControls>,
        ReadStorage<'a, ControledByInput>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (input, controlled, mut vel): Self::SystemData) {
        (&controlled, &mut vel).join().for_each(|(_ctrled, vel)| {
            if input.left {
                vel.x(-200.0);
            } else if input.right {
                vel.x(200.0);
            } else {
                vel.x(0.0);
            }
            if input.up {
                vel.y(-200.0);
            } else if input.down {
                vel.y(200.0);
            } else {
                vel.y(0.0);
            }
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
    type SystemData = (
        Read<'a, Map>,
        ReadStorage<'a, AABB>
    );

    fn run(&mut self, (map, aabb): Self::SystemData) {
        aabb.join().for_each(|aabb| {
            graphics::rectangle(
                self.ctx,
                graphics::DrawMode::Line(1.0),
                graphics::Rect::new(
                    aabb.center.x - aabb.halfsize.x,
                    aabb.center.y - aabb.halfsize.y,
                    aabb.fullsize.x,
                    aabb.fullsize.y
                )
            ).unwrap();
        });

        for (y, x_vec) in map.tiles.iter().enumerate() {
            for (x, tile) in x_vec.iter().enumerate() {
                if tile == &TileType::Block {
                    let tile_pos = map.get_map_tile_position(x as f32, y  as f32);
                    graphics::rectangle(
                        self.ctx,
                        graphics::DrawMode::Line(1.0),
                        graphics::Rect::new(
                            tile_pos.x,
                            tile_pos.y,
                            map.tile_size,
                            map.tile_size
                        )
                    ).unwrap();
                }
            }
        }
    }
}


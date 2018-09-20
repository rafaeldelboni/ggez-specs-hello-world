use ggez::graphics;
use ggez::{Context};
use specs::{System, WriteStorage, Read, ReadStorage, Join};

use resources::{DeltaTime, InputControls, Map, TileType};
use components::{AABB, ControledByInput, Position, Velocity};

pub struct MoveSystem;
impl MoveSystem {
    pub fn tilemap_collision(map: &Map, pos: &Position, aabb: &AABB) -> bool {
        let ab_left = pos.current.x;
        let ab_right = pos.current.x + aabb.fullsize.x;
        let ab_up = pos.current.y;
        let ab_down = pos.current.y + aabb.fullsize.y;

        let mut left_tile = (ab_left / map.tile_size as f32) as isize;
        let mut right_tile = (ab_right / map.tile_size as f32) as isize;
        let mut top_tile = (ab_up / map.tile_size as f32) as isize;
        let mut down_tile = (ab_down / map.tile_size as f32) as isize;

        if left_tile < 0 { left_tile = 0; }
        if right_tile > map.width as isize { right_tile = map.width as isize; }
        if top_tile < 0 { top_tile = 0; }
        if down_tile > map.height as isize { down_tile = map.height as isize; }

        for tile_index_x in left_tile..right_tile+1 {
            for tile_index_y in top_tile..down_tile+1 {
                if map.is_obstacle(tile_index_x as isize, tile_index_y as isize) {
                    let tile_index = map.get_map_tile_position(
                        tile_index_x as f32,
                        tile_index_y as f32
                    );

                    let tl_left = tile_index.x;
                    let tl_right = tile_index.x + map.tile_size;
                    let tl_up = tile_index.y;
                    let tl_down = tile_index.y + map.tile_size;

                    let left_colide = ab_left < tl_right;
                    let right_colide = ab_right > tl_left;
                    let up_colide = ab_up < tl_down;
                    let down_colide = ab_down > tl_up;

                    let collision = left_colide &&
                        right_colide &&
                        up_colide &&
                        down_colide;

                    if collision {
                        return true;
                    }
                }
            }
        }
        return false;
    }
}
impl<'a> System<'a> for MoveSystem {
    type SystemData = (
        Read<'a, Map>,
        Read<'a, DeltaTime>,
        WriteStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, AABB>
    );

    fn run(&mut self, (map, delta, mut vel, mut pos, mut aabb): Self::SystemData) {
        (&mut vel, &mut pos, &mut aabb).join().for_each(|(vel, pos, aabb)| {
            let delta_vel = vel.get() * delta.0;

            let mut old_pos = pos.current.clone();
            pos.x(delta_vel.x);
            if MoveSystem::tilemap_collision(&map, pos, aabb) {
                vel.x(0.0);
                pos.set_x(old_pos.x);
            }

            old_pos = pos.current.clone();
            pos.y(delta_vel.y);
            if MoveSystem::tilemap_collision(&map, pos, aabb) {
                vel.y(0.0);
                pos.set_y(old_pos.y);
            }

            aabb.set_center(pos.current);
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
                vel.x(-300.0);
            } else if input.right {
                vel.x(300.0);
            } else {
                vel.x(0.0);
            }
            if input.up {
                vel.y(-300.0);
            } else if input.down {
                vel.y(300.0);
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
            graphics::rectangle( self.ctx,
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


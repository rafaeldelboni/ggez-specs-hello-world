use ggez::graphics;

use specs::{Component, NullStorage, VecStorage, World};

pub fn register_components(world: &mut World) {
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Controlable>();
    world.register::<AABB>();
}

#[derive(Debug)]
pub struct Position {
    pub old: graphics::Vector2,
    pub current: graphics::Vector2,
}
impl Component for Position {
    type Storage = VecStorage<Self>;
}
impl Position {
    pub fn new(initial: graphics::Vector2) -> Position {
        Position {
            old: initial.clone(),
            current: initial.clone(),
        }
    }

    pub fn y(&mut self, y: f32) {
        self.old.y = self.current.y;
        self.current.y += y.round();
    }

    pub fn x(&mut self, x: f32) {
        self.old.x = self.current.x;
        self.current.x += x.round();
    }
}

#[derive(Debug)]
pub struct Velocity {
    pub old: graphics::Vector2,
    pub current: graphics::Vector2,
}
impl Component for Velocity {
    type Storage = VecStorage<Self>;
}
impl Velocity {
    pub fn new(initial: graphics::Vector2) -> Velocity {
        Velocity {
            old: initial.clone(),
            current: initial.clone(),
        }
    }

    pub fn get(&self) -> graphics::Vector2 {
        if self.current.x != 0.0 && self.current.y != 0.0 {
            let pi_inverse = 1.0 / (2.0 as f32).sqrt();
            self.current * pi_inverse
        } else {
            self.current
        }
    }

    pub fn y(&mut self, y: f32) {
        self.old.y = self.current.y;
        self.current.y = y;
    }

    pub fn x(&mut self, x: f32) {
        self.old.x = self.current.x;
        self.current.x = x;
    }
}

#[derive(Debug, Default)]
pub struct Controlable;
impl Component for Controlable {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct AABB {
    pub center: graphics::Vector2,
    pub fullsize: graphics::Vector2,
    pub halfsize: graphics::Vector2,
    pub offset: graphics::Vector2,
    pub pushed_right_wall: bool,
    pub pushes_right_wall: bool,
    pub pushed_left_wall: bool,
    pub pushes_left_wall: bool,
    pub pushed_up_wall: bool,
    pub pushes_up_wall: bool,
    pub pushed_down_wall: bool,
    pub pushes_down_wall: bool,
}
impl Component for AABB {
    type Storage = VecStorage<Self>;
}
impl AABB {
    pub fn new(
        fullsize: graphics::Vector2,
        center: graphics::Vector2,
        offset: graphics::Vector2
    ) -> AABB {
        AABB {
            halfsize: fullsize / 2.0,
            fullsize: fullsize,
            center: center,
            offset: offset,
            pushed_right_wall: false,
            pushes_right_wall: false,
            pushed_left_wall: false,
            pushes_left_wall: false,
            pushed_up_wall: false,
            pushes_up_wall: false,
            pushed_down_wall: false,
            pushes_down_wall: false,
        }
    }

    pub fn set_center(&mut self, center: graphics::Vector2) {
        self.center = center + self.offset;
    }

    pub fn set_pushes_right_wall(&mut self, pushes_right_wall: bool) {
        self.pushed_right_wall = self.pushes_right_wall;
        self.pushes_right_wall = pushes_right_wall;
    }
    pub fn set_pushes_left_wall(&mut self, pushes_left_wall: bool) {
        self.pushed_left_wall = self.pushes_left_wall;
        self.pushes_left_wall = pushes_left_wall;
    }
    pub fn set_pushes_up_wall(&mut self, pushes_up_wall: bool) {
        self.pushed_up_wall = self.pushes_up_wall;
        self.pushes_up_wall = pushes_up_wall;
    }
    pub fn set_pushes_down_wall(&mut self, pushes_down_wall: bool) {
        self.pushed_down_wall = self.pushes_down_wall;
        self.pushes_down_wall = pushes_down_wall;
    }

    pub fn overlaps(&self, other: AABB) -> bool {
        if (self.center.x - other.center.x).abs() >
            self.halfsize.x + other.halfsize.x {
                return false;
            }
        if (self.center.y - other.center.y).abs() >
            self.halfsize.y + other.halfsize.y {
                return false;
            }
        return true;
    }
}

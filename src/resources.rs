use ggez::graphics;

#[derive(Debug, Default)]
pub struct DeltaTime(pub f32);

#[derive(Clone)]
pub struct InputControls {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}
impl InputControls {
    pub fn new() -> InputControls {
        InputControls {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}
impl Default for InputControls {
    fn default() -> InputControls {
        InputControls {
            up: false,
            down: false,
            left: false,
            right: false,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TileType {
    Empty,
    Block,
    Slope45,
}

#[derive(Debug, Default)]
pub struct PositionVector {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Default)]
pub struct Map {
    pub tiles: Vec<Vec<TileType>>,
    pub position: PositionVector,
    pub width: usize,
    pub height: usize,
    pub tile_size: f32,
}
impl Map {
    pub fn demo() -> Map {
        Map {
            tiles: vec![
                vec![ TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block, TileType::Slope45, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Slope45, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Slope45, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block, TileType::Block ],
            ],
            position: PositionVector { x: 0.0, y: 0.0 },
            width: 25,
            height: 19,
            tile_size: 32.0,
        }
    }

    pub fn get_tile_at_point(&self, point: graphics::Point2) -> graphics::Point2 {
        graphics::Point2::new(
            self.get_tile_x_at_point(point.x) as f32,
            self.get_tile_y_at_point(point.y) as f32,
        )
    }

    pub fn get_tile_y_at_point(&self, y: f32) -> isize {
        ((y - self.position.y) / self.tile_size) as isize
    }

    pub fn get_tile_x_at_point(&self, x: f32) -> isize {
        ((x - self.position.x) / self.tile_size) as isize
    }

    pub fn get_map_tile_position(&self, x: f32, y: f32) -> graphics::Vector2 {
        graphics::Vector2::new(
            self.get_map_tile_x_position(x),
            self.get_map_tile_y_position(y)
        )
    }

    pub fn get_map_tile_x_position(&self, x: f32) -> f32 {
        x as f32 * self.tile_size + self.position.x
    }

    pub fn get_map_tile_y_position(&self, y: f32) -> f32 {
        y as f32 * self.tile_size + self.position.y
    }

    pub fn get_map_tile_position_vec(&self, coords: graphics::Point2)
        -> graphics::Vector2 {
            self.get_map_tile_position(coords.x, coords.y)
        }

    fn in_bounds(&self, x: isize, y: isize) -> Option<(usize, usize)> {
        if x < 0 || x as usize >= self.width || y < 0 || y as usize >= self.height {
            None
        } else {
            Some((x as usize, y as usize))
        }
    }

    pub fn get_tile(&self, x: isize, y: isize) -> TileType {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.tiles[y][x]
        } else {
            TileType::Block
        }
    }

    pub fn is_obstacle(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) == TileType::Block
    }

    pub fn is_slope(&self, x: isize, y: isize) -> bool {
        self.get_tile(x, y) == TileType::Slope45
    }

    pub fn is_empty(&self, x: isize, y: isize) -> bool {
        if let Some((x, y)) = self.in_bounds(x, y) {
            self.tiles[y][x] == TileType::Empty
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use ggez::graphics::{Point2, Vector2};
    use resources::{TileType, Map};

    fn generate_map() -> Map {
        Map {
            tiles: vec![
                vec![ TileType::Block, TileType::Empty, TileType::Block ],
                vec![ TileType::Block, TileType::Empty, TileType::Block ],
            ],
            position: PositionVector {x: 0.0, y: 0.0},
            width: 3,
            height: 2,
            tile_size: 16.0,
        }
    }

    #[test]
    fn should_get_tile_at_point() {
        let map = generate_map();
        assert_eq!(
            map.get_tile_at_point(Point2::new(22.0, 22.0)),
            Point2::new(1.0, 1.0)
        );
        assert_eq!(
            map.get_tile_at_point(Point2::new(2.0, 16.0)),
            Point2::new(0.0, 1.0)
        );
        assert_eq!(
            map.get_tile_at_point(Point2::new(32.0, 20.0)),
            Point2::new(2.0, 1.0)
        );
    }

    #[test]
    fn should_get_map_tile_position_vec() {
        let map = generate_map();
        assert_eq!(
            map.get_map_tile_position_vec(Point2::new(0.0, 0.0)),
            Vector2::new(0.0, 0.0)
        );
        assert_eq!(
            map.get_map_tile_position_vec(Point2::new(0.0, 1.0)),
            Vector2::new(0.0, 16.0)
        );
        assert_eq!(
            map.get_map_tile_position_vec(Point2::new(2.0, 1.0)),
            Vector2::new(32.0, 16.0)
        );
    }

    #[test]
    fn should_be_in_bounds() {
        let map = generate_map();
        assert_eq!(map.in_bounds(0, 0), Some((0, 0)));
        assert_eq!(map.in_bounds(0, 1), Some((0, 1)));
        assert_eq!(map.in_bounds(2, 1), Some((2, 1)));
    }

    #[test]
    fn should_not_be_in_bounds() {
        let map = generate_map();
        assert_eq!(map.in_bounds(3, 3), None);
        assert_eq!(map.in_bounds(-1, 4), None);
    }

    #[test]
    fn should_get_tile_block() {
        let map = generate_map();
        assert_eq!(map.get_tile(3, 3), TileType::Block);
        assert_eq!(map.get_tile(0, 0), TileType::Block);
        assert_eq!(map.get_tile(1, 2), TileType::Block);
    }

    #[test]
    fn should_get_tile_empty() {
        let map = generate_map();
        assert_eq!(map.get_tile(1, 0), TileType::Empty);
        assert_eq!(map.get_tile(1, 1), TileType::Empty);
    }

    #[test]
    fn should_check_if_is_obstacle() {
        let map = generate_map();
        assert_eq!(map.is_obstacle(3, 3), true);
        assert_eq!(map.is_obstacle(0, 0), true);
        assert_eq!(map.is_obstacle(1, 2), true);
        assert_eq!(map.is_obstacle(1, 0), false);
        assert_eq!(map.is_obstacle(1, 1), false);
    }

    #[test]
    fn should_check_if_is_empty() {
        let map = generate_map();
        assert_eq!(map.is_empty(3, 3), false);
        assert_eq!(map.is_empty(0, 0), false);
        assert_eq!(map.is_empty(1, 2), false);
        assert_eq!(map.is_empty(1, 0), true);
        assert_eq!(map.is_empty(1, 1), true);
    }
}

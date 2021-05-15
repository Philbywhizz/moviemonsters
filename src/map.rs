use crate::prelude::*;

pub const BLOCK_SIZE: i32 = 9;

pub const MAP_WIDTH: i32 = 9 * BLOCK_SIZE; //81
pub const MAP_HEIGHT: i32 = 9 * BLOCK_SIZE; //81

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Ground,
    Building,
}

// Structure that will hold a map
pub struct Map {
    pub tiles: Vec<TileType>,
    pub center: Point,
}

impl Map {
    pub fn new() -> Self {
        Self {
            // just create an empty map
            tiles: vec![TileType::Ground; NUM_TILES],
            center: Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2),
        }
    }

    // Test if we are currently on the map
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    // Test that the requested tile is free
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Ground
    }

    /// Will randomly pick a tile and only return it if it is currently empty
    pub fn get_random_empty_tile_location(&self) -> Point {
        let mut rng = RandomNumberGenerator::new();
        // TODO: There is a risk of this being blocking code, need to rethink this
        loop {
            let test_location = Point::new(rng.range(0, MAP_WIDTH), rng.range(0, MAP_HEIGHT));
            if self.can_enter_tile(test_location) {
                return test_location;
            }
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }

    fn in_bounds(&self, point: Point) -> bool {
        self.in_bounds(point)
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

// Return the index value in the tile vector
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

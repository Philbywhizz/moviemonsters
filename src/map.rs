use crate::prelude::*;

pub const MAP_WIDTH: i32 = 100;
pub const MAP_HEIGHT: i32 = 100;

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Ground,
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

    // pub fn try_idx(&self, point: Point) -> Option<usize> {
    //     if !self.in_bounds(point) {
    //         None
    //     } else {
    //         Some(map_idx(point.x, point.y))
    //     }
    // }
}

// Return the index value in the tile vector
pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

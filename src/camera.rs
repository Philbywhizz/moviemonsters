use crate::prelude::*;

// The camera shows only part of the world map, defined by its size and look_at point

pub const CAMERA_WIDTH: i32 = 57;
pub const CAMERA_HEIGHT: i32 = 45;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(look_at: Point) -> Self {
        Self {
            left_x: look_at.x - CAMERA_WIDTH / 2,
            right_x: look_at.x + CAMERA_WIDTH / 2,
            top_y: look_at.y - CAMERA_HEIGHT / 2,
            bottom_y: look_at.y + CAMERA_HEIGHT / 2,
        }
    }

    pub fn update(&mut self, look_at: Point) {
        self.left_x = look_at.x - CAMERA_WIDTH / 2;
        self.right_x = look_at.x + CAMERA_WIDTH / 2;
        self.top_y = look_at.y - CAMERA_HEIGHT / 2;
        self.bottom_y = look_at.y + CAMERA_HEIGHT / 2;
    }
}

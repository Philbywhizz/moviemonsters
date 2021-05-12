use crate::prelude::*;

pub const CAMERA_WIDTH: i32 = 50;
pub const CAMERA_HEIGHT: i32 = 40;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(focus: Point) -> Self {
        Self {
            left_x: focus.x - CAMERA_WIDTH / 2,
            right_x: focus.x + CAMERA_WIDTH / 2,
            top_y: focus.y - CAMERA_HEIGHT / 2,
            bottom_y: focus.y + CAMERA_HEIGHT / 2,
        }
    }

    pub fn update(&mut self, focus: Point) {
        self.left_x = focus.x - CAMERA_WIDTH / 2;
        self.right_x = focus.x + CAMERA_WIDTH / 2;
        self.top_y = focus.y - CAMERA_HEIGHT / 2;
        self.bottom_y = focus.y + CAMERA_HEIGHT / 2;
    }
}

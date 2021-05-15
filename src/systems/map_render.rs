use crate::prelude::*;

// system to draw the current map to the screen
#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // start the batching
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0); // draw on main console

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let camera_offset = Point::new(camera.left_x, camera.top_y);
            let viewport_offset = Point::new(1, 1);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Ground => 250, //cp437 250
                    TileType::Building => to_cp437('#'),
                };
                draw_batch.set(
                    pt - camera_offset + viewport_offset,
                    ColorPair::new(WHITE, BLACK),
                    glyph,
                );
            }
        }
    }

    draw_batch.submit(0).expect("Batch Error");
}

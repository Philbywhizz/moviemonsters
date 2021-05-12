use crate::prelude::*;

// system to draw the current map to the screen
#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..=camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);
            if map.in_bounds(pt) {
                let idx = map_idx(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Ground => to_cp437('.'),
                };
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // for y in 0..MAP_HEIGHT {
    //     for x in 0..MAP_WIDTH {
    //         let idx = map_idx(x, y);
    //         let glyph = match map.tiles[idx] {
    //             TileType::Ground => to_cp437('.'),
    //         };
    //         draw_batch.set(Point::new(x, y), ColorPair::new(WHITE, BLACK), glyph);
    //     }
    // }
    draw_batch.submit(0).expect("Batch Error");
}

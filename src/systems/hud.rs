use crate::prelude::*;

// A simple HUD

#[system]
#[read_component(Monster)]
pub fn hud() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Main Map is the left most 60 columns leaving 58 usable columns
    // draw_batch.draw_double_box(
    //     Rect::with_size(0, 0, 59, 46),
    //     ColorPair::new(DARKBLUE, BLACK),
    // );

    // Info Panel is the right most 20 columns leaving 18 usable columns
    draw_batch.draw_double_box(
        Rect::with_size(60, 0, 19, 46),
        ColorPair::new(DARKBLUE, BLACK),
    );

    // Bottom Footer are the bottom 3 lines of the screen leaving 1 usable row
    draw_batch.draw_double_box(
        Rect::with_size(0, 47, 79, 2),
        ColorPair::new(DARKBLUE, BLACK),
    );

    draw_batch.submit(10000).expect("Batch Error");
}

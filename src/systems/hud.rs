use crate::prelude::*;

// A simple HUD

#[system]
#[read_component(Monster)]
pub fn hud() {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);

    // Main Map is the left most 60 columns
    draw_batch.draw_double_box(
        Rect::with_size(0, 0, 59, 46),
        ColorPair::new(DARKBLUE, BLACK),
    );
    // draw_batch.print_color(
    //     Point::new(1, 1),
    //     "1234567890123456789012345678901234567890123456789012345678", // 58
    //     ColorPair::new(DIMGREY, BLACK),
    // );

    // Info Panel is the right most 20 columns
    draw_batch.draw_double_box(
        Rect::with_size(60, 0, 19, 46),
        ColorPair::new(DARKBLUE, BLACK),
    );
    draw_batch.print_color(
        Point::new(61, 1),
        "123456789012345678", //18
        ColorPair::new(DIMGREY, BLACK),
    );

    // Bottom Footer are the bottom 3 lines of the screen
    draw_batch.draw_double_box(
        Rect::with_size(0, 47, 79, 2),
        ColorPair::new(DARKBLUE, BLACK),
    );
    draw_batch.print_color(
        Point::new(1, 48),
        "123456789012345678901234567890123456789012345678901234567890123456789012345678", //78
        ColorPair::new(DIMGREY, BLACK),
    );

    draw_batch.print_color_centered(0, "Movie Monsters", ColorPair::new(WHITE, BLACK));

    draw_batch.submit(10000).expect("Batch Error");
}

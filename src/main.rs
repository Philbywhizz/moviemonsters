mod prelude {
    pub use bracket_lib::prelude::*;
}

use prelude::*;

struct State {}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, format!("FPS: {}", ctx.fps));
        ctx.print_centered(1, "Movie Monsters");
    }
}

// Main entry point
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Movie Monsters")
        .with_dimensions(80, 50)
        .with_fps_cap(30.0)
        .build()?;

    let gs: State = State {};

    main_loop(context, gs)
}

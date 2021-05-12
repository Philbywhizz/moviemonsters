mod components;
mod spawner;
mod systems;
mod prelude {
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    // Create our game state
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();

        spawn_monster(&mut ecs, Point::new(10, 10));
        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(0, 0, format!("FPS: {}", ctx.fps));
        ctx.print_centered(1, "Movie Monsters");

        if let Some(key) = ctx.key {
            match key {
                // ESCAPE key aborts
                VirtualKeyCode::Escape => ctx.quitting = true,
                // Everything else pass as a resource
                _ => self.resources.insert(key),
            }
        }

        // Execute the Scheduler
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render a frame
        render_draw_buffer(ctx).expect("Render Error");
    }
}

// Main entry point
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Movie Monsters")
        .with_dimensions(80, 50)
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}

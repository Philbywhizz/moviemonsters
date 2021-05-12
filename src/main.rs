mod camera;
mod components;
mod map;
mod spawner;
mod systems;
mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
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
        let map = Map::new();
        // Insert the map of the city
        resources.insert(map);
        resources.insert(Camera::new(Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2)));

        // spawn the monster in the middle of the map
        spawn_monster(&mut ecs, Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2));

        Self {
            ecs,
            resources,
            systems: build_scheduler(),
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // clear all the consoles
        ctx.set_active_console(0);
        ctx.cls(); // Map
        ctx.set_active_console(1);
        ctx.cls(); // Entities
        ctx.set_active_console(2);
        ctx.cls(); // HUD

        if let Some(key) = ctx.key {
            match key {
                // ESCAPE key aborts
                VirtualKeyCode::Escape => ctx.quitting = true,
                _ => {}
            }
        }

        self.resources.insert(ctx.key);
        // Execute the Scheduler
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render a frame on all the consoles
        render_draw_buffer(ctx).expect("Render Error");
    }
}

// Main entry point
fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Movie Monsters")
        .with_fps_cap(30.0)
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_resource_path("resources/")
        .with_font("terminal8x8.png", 8, 8)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Main screen
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // Entities
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "terminal8x8.png") // HUD
        .build()?;

    main_loop(context, State::new())
}

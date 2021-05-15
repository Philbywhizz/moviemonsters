mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;
mod prelude {
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
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
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // spawn some random actors for testing
        for _ in 1..25 {
            spawn_actor(&mut ecs, map_builder.map.get_random_empty_tile_location());
        }

        let monster_location = map_builder.map.get_random_empty_tile_location();
        spawn_monster(&mut ecs, monster_location);

        // Insert the map of the city as a resource
        resources.insert(map_builder.map);

        // Add a Camera as a resource, pointing at the monster
        resources.insert(Camera::new(monster_location));

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
        .with_font("md-curses-16x16.png", 16, 16)
        .with_simple_console(SCREEN_WIDTH, SCREEN_HEIGHT, "md-curses-16x16.png") // Main screen
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "md-curses-16x16.png") // Entities
        .with_simple_console_no_bg(SCREEN_WIDTH, SCREEN_HEIGHT, "md-curses-16x16.png") // HUD
        .build()?;

    main_loop(context, State::new())
}

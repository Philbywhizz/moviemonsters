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
        ctx.cls();

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

        // Render a frame
        render_draw_buffer(ctx).expect("Render Error");
    }
}

// Main entry point
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Movie Monsters")
        .with_dimensions(SCREEN_WIDTH, SCREEN_HEIGHT)
        .with_fps_cap(30.0)
        .build()?;

    main_loop(context, State::new())
}

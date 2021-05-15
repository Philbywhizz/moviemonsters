use crate::prelude::*;

// Spawn a monster at a given location in the world
pub fn spawn_monster(ecs: &mut World, pos: Point) {
    ecs.push((
        Monster,
        pos,
        Render {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: to_cp437('█'),
        },
    ));
}

pub fn spawn_actor(ecs: &mut World, pos: Point) {
    ecs.push((
        Actor,
        pos,
        Render {
            color: ColorPair::new(CYAN, BLACK),
            glyph: 1,
        },
        RandomMovement { speed: 300 },
    ));
}

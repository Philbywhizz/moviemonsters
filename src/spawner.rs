use crate::prelude::*;

// Spawn a monster at a given location in the world
pub fn spawn_monster(ecs: &mut World, pos: Point) {
    ecs.push((
        Monster,
        pos,
        Render {
            color: ColorPair::new(YELLOW, BLACK),
            glyph: 219,
        },
    ));
}

use crate::prelude::*;

// Collision system

#[system]
#[read_component(Point)]
#[read_component(Monster)]
#[read_component(Actor)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut monster_pos = Point::zero();
    let mut monsters = <&Point>::query().filter(component::<Monster>());
    monsters.iter(ecs).for_each(|pos| monster_pos = *pos);

    let mut actors = <(Entity, &Point)>::query().filter(component::<Actor>());

    // If we touch an actor then remove it from the game world
    actors
        .iter(ecs)
        .filter(|(_, pos)| **pos == monster_pos)
        .for_each(|(entity, _)| {
            commands.remove(*entity);
        });
}

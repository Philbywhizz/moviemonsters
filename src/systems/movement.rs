use crate::prelude::*;

#[system(for_each)]
#[read_component(Monster)]
pub fn movement(
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
) {
    // function runs on every 'movement' message
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        if ecs
            .entry_ref(want_move.entity)
            .unwrap()
            .get_component::<Monster>()
            .is_ok()
        {
            camera.update(want_move.destination);
        }
    }

    // remove the message
    commands.remove(*entity);
}

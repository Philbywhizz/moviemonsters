use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(RandomMovement)]
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut movers = <(Entity, &Point, &mut RandomMovement)>::query();

    movers.iter_mut(ecs).for_each(|(entity, pos, rnd_move)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // up
            1 => Point::new(1, 0),  // down
            2 => Point::new(0, -1), // left
            _ => Point::new(0, 1),  // right
        } + *pos;

        rnd_move.current_step += 1;

        // only move when we have reached our max steps
        if rnd_move.current_step >= rnd_move.max_step {
            commands.push((
                (),
                WantsToMove {
                    entity: *entity,
                    destination,
                },
            ));
            rnd_move.current_step = 0;
        }
    });
}

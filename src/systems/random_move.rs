use crate::prelude::*;

#[system]
#[write_component(Point)]
#[write_component(RandomMovement)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map, #[resource] dt: &f32) {
    let mut movers = <(&mut Point, &mut RandomMovement)>::query();

    movers.iter_mut(ecs).for_each(|(pos, rnd_move)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // up
            1 => Point::new(1, 0),  // down
            2 => Point::new(0, -1), // left
            _ => Point::new(0, 1),  // right
        } + *pos;

        rnd_move.current_delta += dt;

        if rnd_move.current_delta >= rnd_move.trigger_delta && map.can_enter_tile(destination) {
            *pos = destination;
            rnd_move.current_delta = 0.0;
        }
    });
}

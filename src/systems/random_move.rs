use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(RandomMovement)]
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    let mut movers = <(&mut Point, &RandomMovement)>::query();

    movers.iter_mut(ecs).for_each(|(pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0, 4) {
            0 => Point::new(-1, 0), // up
            1 => Point::new(1, 0),  // down
            2 => Point::new(0, -1), // left
            _ => Point::new(0, 1),  // right
        } + *pos;

        if map.can_enter_tile(destination) {
            *pos = destination;
        }
    });
}

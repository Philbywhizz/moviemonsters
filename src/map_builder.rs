use crate::prelude::*;
const NUM_BUILDINGS: usize = 80;

pub struct MapBuilder {
    pub map: Map,
    pub buildings: Vec<Rect>,
    pub monster_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder {
            map: Map::new(),
            buildings: Vec::new(),
            monster_start: Point::zero(),
        };
        mb.fill(TileType::Ground);
        mb.make_random_buildings(rng);
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn make_random_buildings(&mut self, rng: &mut RandomNumberGenerator) {
        while self.buildings.len() < NUM_BUILDINGS {
            let building = Rect::with_size(
                rng.range(1, MAP_WIDTH - 10),
                rng.range(1, MAP_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );
            let mut overlap = false;
            for b in self.buildings.iter() {
                if b.intersect(&building) {
                    overlap = true;
                }
            }

            if !overlap {
                building.for_each(|p| {
                    if p.x > 0 && p.x < MAP_WIDTH && p.y > 0 && p.y < MAP_HEIGHT {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Building;
                    }
                });
                self.buildings.push(building);
            }
        }
    }
}

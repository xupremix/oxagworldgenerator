use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::worldgenerator::Generator;
use std::ops::Range;

pub struct WorldGenerationOptions {
    sea_level: Range<f32>,
    sand_level: Range<f32>,
    grass_level: Range<f32>,
    street_level: Range<f32>,
}

pub struct WorldGenerator {
    size: usize,
    seed: usize,
    world_gen_options: WorldGenerationOptions,
}

impl WorldGenerator {
    pub fn init(size: usize, seed: Option<usize>) -> Self {
        let mut rng = rand::thread_rng();
        let seed = match seed {
            Some(seed) => seed,
            None => {
                rng.gen();
            }
        };
        let world_gen_options = WorldGenerationOptions {
            sea_level: 0.0..0.0,
            sand_level: 0.0..0.0,
            grass_level: 0.0..0.0,
            street_level: 0.0..0.0,
        };
        Self {
            size,
            seed,
            world_gen_options,
        }
    }
}

impl Generator for WorldGenerator {
    fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        todo!()
    }
}

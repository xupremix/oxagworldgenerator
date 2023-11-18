pub mod world_gen_options;

use crate::utils::{OxAgError, DEFAULT_WORLD_SIZE};
use crate::worldgenerator::world_gen_options::OxAgWorldGenerationOptions;
use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::worldgenerator::Generator;

pub struct OxAgWorldGenerator {
    size: usize,
    seed: u32,
    world_gen_options: OxAgWorldGenerationOptions,
}

impl OxAgWorldGenerator {
    pub fn new(seed: u32) -> OxAgWorldGenerator {
        OxAgWorldGenerator {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: Default::default(),
        }
    }
    pub fn set_seed(mut self, seed: u32) -> OxAgWorldGenerator {
        self.seed = seed;
        self
    }
    pub fn set_size(mut self, size: usize) -> OxAgWorldGenerator {
        self.size = size;
        self
    }
    pub fn set_world_gen_options(
        mut self,
        world_gen_options: OxAgWorldGenerationOptions,
    ) -> Result<OxAgWorldGenerator, OxAgError> {
        if world_gen_options.is_valid() {
            self.world_gen_options = world_gen_options;
            Ok(self)
        } else {
            Err(OxAgError::InvalidWorldGenerationOption)
        }
    }
}

impl Default for OxAgWorldGenerator {
    fn default() -> OxAgWorldGenerator {
        OxAgWorldGenerator {
            size: DEFAULT_WORLD_SIZE,
            seed: rand::thread_rng().gen::<u32>(),
            world_gen_options: Default::default(),
        }
    }
}

impl Generator for OxAgWorldGenerator {
    fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        todo!()
    }
}

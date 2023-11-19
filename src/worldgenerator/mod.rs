pub mod world_gen_options;

use crate::utils::{OxAgError, DEFAULT_WORLD_SIZE};
use crate::worldgenerator::world_gen_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};
use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::worldgenerator::Generator;

#[derive(Debug)]
pub struct OxAgWorldGenerator {
    size: usize,
    seed: u32,
    world_gen_options: OxAgWorldGenerationOptions,
}

impl OxAgWorldGenerator {
    pub fn init() -> Self {
        let seed = rand::thread_rng().gen::<u32>();
        Self {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: OxAgWorldGenerationOptions::new(seed),
        }
    }
    pub fn new(seed: u32) -> Self {
        Self {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: OxAgWorldGenerationOptions::new(seed),
        }
    }
    pub fn set_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self.world_gen_options = OxAgWorldGenerationOptions::new(seed);
        self
    }
    pub fn set_size(mut self, size: usize) -> Self {
        self.size = size;
        self
    }
    pub fn set_world_gen_options(
        mut self,
        world_gen_options: OxAgWorldGenerationOptions,
    ) -> Result<Self, OxAgError> {
        if world_gen_options.is_valid() {
            self.world_gen_options = world_gen_options;
            Ok(self)
        } else {
            Err(OxAgError::InvalidWorldGenerationOption)
        }
    }
    pub fn gen_options_from_preset(mut self, preset: OxAgWorldGenerationPresets) -> Self {
        self.world_gen_options = OxAgWorldGenerationOptions::from_preset(preset);
        self
    }
    pub fn set_content_gen_options() {
        todo!()
    }
}

impl Generator for OxAgWorldGenerator {
    fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        todo!()
    }
}

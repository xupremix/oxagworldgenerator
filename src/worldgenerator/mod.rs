pub mod content_gen_options;
pub mod world_gen_options;

use crate::utils::OxAgError::InvalidContentGenerationOption;
use crate::utils::{OxAgError, DEFAULT_WORLD_SIZE};
use crate::worldgenerator::content_gen_options::{OxAgContentGenerationPresets, OxAgContentOption};
use crate::worldgenerator::world_gen_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};
use rand::Rng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::worldgenerator::Generator;
use std::collections::HashMap;
use std::hash::Hash;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct OxAgWorldGenerator {
    size: usize,
    seed: u32,
    world_gen_options: OxAgWorldGenerationOptions,
    content_gen_options: HashMap<Content, OxAgContentOption>,
}

impl OxAgWorldGenerator {
    pub fn init() -> Self {
        let seed = rand::thread_rng().gen::<u32>();
        Self {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: OxAgWorldGenerationOptions::new(seed),
            content_gen_options: OxAgContentOption::new(seed),
        }
    }
    pub fn new(seed: u32) -> Self {
        Self {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: OxAgWorldGenerationOptions::new(seed),
            content_gen_options: OxAgContentOption::new(seed),
        }
    }
    pub fn set_seed(mut self, seed: u32) -> Self {
        self.seed = seed;
        self.world_gen_options = OxAgWorldGenerationOptions::new(seed);
        self.content_gen_options = OxAgContentOption::new(seed);
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
    pub fn set_content_gen_options(
        mut self,
        content_gen_options: HashMap<Content, OxAgContentOption>,
    ) -> Result<Self, OxAgError> {
        for content in Content::iter() {
            match content_gen_options.get(&content) {
                Some(content_option) => {
                    if !content_option.is_valid() {
                        return Err(InvalidContentGenerationOption(content));
                    }
                }
                None => return Err(InvalidContentGenerationOption(content)),
            }
        }
        self.content_gen_options = content_gen_options;
        Ok(self)
    }
    pub fn gen_world_options_from_preset(mut self, preset: OxAgWorldGenerationPresets) -> Self {
        self.world_gen_options = OxAgWorldGenerationOptions::from_preset(preset);
        self
    }
    pub fn gen_content_options_from_preset(mut self, preset: OxAgContentGenerationPresets) -> Self {
        self.content_gen_options = OxAgContentOption::from_preset(preset);
        self
    }
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn get_seed(&self) -> u32 {
        self.seed
    }
    pub fn get_world_gen_options(&self) -> &OxAgWorldGenerationOptions {
        &self.world_gen_options
    }
    pub fn get_content_gen_options(&self) -> &HashMap<Content, OxAgContentOption> {
        &self.content_gen_options
    }
}

impl Generator for OxAgWorldGenerator {
    fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        todo!()
    }
}

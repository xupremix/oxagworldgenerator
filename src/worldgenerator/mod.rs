pub mod content_gen_options;
pub mod world_gen_options;

use crate::utils::OxAgError::{ContentOptionNotSet, InvalidContentGenerationOption};
use crate::utils::{default_from, OxAgError, DEFAULT_WORLD_SIZE};
use crate::worldgenerator::content_gen_options::{OxAgContentGenerationPresets, OxAgContentOption};
use crate::worldgenerator::world_gen_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::{
    Foggy, Rainy, Sunny, TrentinoWinter, TropicalMonsoon,
};
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::worldgenerator::Generator;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::Range;
use strum::IntoEnumIterator;

#[derive(Debug, Clone)]
pub struct OxAgWorldGenerator {
    size: usize,
    seed: u64,
    world_gen_options: OxAgWorldGenerationOptions,
    content_gen_options: HashMap<Content, OxAgContentOption>,
    weather_gen_options: EnvironmentalConditions,
}

impl OxAgWorldGenerator {
    pub fn init() -> Self {
        Self::new(rand::thread_rng().gen::<u64>())
    }
    pub fn new(seed: u64) -> Self {
        Self {
            size: DEFAULT_WORLD_SIZE,
            seed,
            world_gen_options: OxAgWorldGenerationOptions::new(seed),
            content_gen_options: OxAgContentOption::new(seed),
            weather_gen_options: OxAgWorldGenerator::gen_environmental_conditions(seed),
        }
    }
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self.world_gen_options = OxAgWorldGenerationOptions::new(seed);
        self.content_gen_options = OxAgContentOption::new(seed);
        self.weather_gen_options = OxAgWorldGenerator::gen_environmental_conditions(seed);
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
        mut content_gen_options: HashMap<Content, OxAgContentOption>,
    ) -> Result<Self, OxAgError> {
        for content in Content::iter() {
            match &content {
                Content::None => {
                    let _ = content_gen_options.remove(&content);
                }
                other => match content_gen_options.get(&content) {
                    Some(content_option) => {
                        if !content_option.is_valid() {
                            return Err(InvalidContentGenerationOption(content));
                        }
                    }
                    None => return Err(ContentOptionNotSet(content)),
                },
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
    pub fn get_seed(&self) -> u64 {
        self.seed
    }
    pub fn get_world_gen_options(&self) -> &OxAgWorldGenerationOptions {
        &self.world_gen_options
    }
    pub fn get_content_gen_options(&self) -> &HashMap<Content, OxAgContentOption> {
        &self.content_gen_options
    }
    pub fn alter_content_gen_options(
        mut self,
        content: Content,
        content_gen_option: OxAgContentOption,
    ) -> Option<Self> {
        if !content_gen_option.is_valid() || content == Content::None {
            None
        } else {
            self.content_gen_options
                .insert(default_from(content), content_gen_option);
            Some(self)
        }
    }
    pub fn set_environmental_conditions(
        mut self,
        weather_gen_options: EnvironmentalConditions,
    ) -> Self {
        self.weather_gen_options = weather_gen_options;
        self
    }
    fn gen_environmental_conditions(seed: u64) -> EnvironmentalConditions {
        let mut rng = StdRng::seed_from_u64(seed);
        let number = rng.gen::<u8>();
        let mut vec = vec![];
        for _ in 0..=number {
            vec.push(match rng.gen_range::<u8, Range<u8>>(0..4) {
                0 => Sunny,
                1 => Rainy,
                2 => Foggy,
                3 => TropicalMonsoon,
                4 => TrentinoWinter,
                _ => Sunny,
            });
        }
        // TODO implement EnumIter in WeatherType

        EnvironmentalConditions::new(&vec, rng.gen::<u8>(), rng.gen::<u8>())
    }
    pub fn get_weather_gen_options(&self) -> &EnvironmentalConditions {
        &self.weather_gen_options
    }
}

impl Generator for OxAgWorldGenerator {
    fn new(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        todo!()
    }
}

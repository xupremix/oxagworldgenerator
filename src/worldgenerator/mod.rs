use std::collections::hash_map::Entry::{Occupied, Vacant};
use std::collections::HashMap;
use std::hash::Hash;

use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::SeedableRng;
use rand::{thread_rng, Rng};
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::worldgenerator::Generator;
use strum::IntoEnumIterator;

use crate::utils::OxAgError::{
    CannotSetContentOptionForNone, ContentOptionNotSet, ContentOptionsNotSet, SeedNotSet,
    SizeNotSet, WeatherOptionsNotSet, WorldOptionsNotSet,
};
use crate::utils::{
    OxAgError, DEFAULT_NOISE_FREQUENCY, DEFAULT_NOISE_LACUNARITY, DEFAULT_NOISE_OCTAVES,
    DEFAULT_NOISE_PERSISTANCE,
};
use crate::worldgenerator::content_options::{OxAgContentGenerationPresets, OxAgContentOption};
use crate::worldgenerator::environmental_condition_options::{
    EnvironmentalConditionsPresets, OxAgEnvironmentalConditionsOptions,
};
use crate::worldgenerator::world_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};

pub mod content_options;
pub mod environmental_condition_options;
pub mod world_options;

pub struct OxAgWorldGeneratorBuilder {
    size: Option<usize>,
    seed: Option<u64>,
    world_options: Option<OxAgWorldGenerationOptions>,
    content_options: Option<HashMap<Content, OxAgContentOption>>,
    environmental_conditions: Option<EnvironmentalConditions>,
}

#[derive(Debug, Clone)]
pub struct OxAgWorldGenerator {
    size: usize,
    seed: u64,
    world_options: OxAgWorldGenerationOptions,
    content_options: HashMap<Content, OxAgContentOption>,
    environmental_conditions: EnvironmentalConditions,
}

impl OxAgWorldGeneratorBuilder {
    pub fn build(&self) -> OxAgWorldGenerator {
        let seed = self.seed.unwrap_or(thread_rng().gen::<u64>());
        OxAgWorldGenerator {
            size: self
                .size
                .unwrap_or(StdRng::seed_from_u64(seed).gen_range(0..u8::MAX) as usize),
            seed: seed,
            world_options: self
                .world_options
                .clone()
                .unwrap_or(OxAgWorldGenerationOptions::new(seed)),
            environmental_conditions: self
                .environmental_conditions
                .clone()
                .unwrap_or(OxAgEnvironmentalConditionsOptions::new(seed)),
            content_options: self
                .content_options
                .clone()
                .unwrap_or(OxAgContentOption::new(seed)),
        }
    }
    pub fn new() -> Self {
        Self {
            size: None,
            seed: None,
            world_options: None,
            content_options: None,
            environmental_conditions: None,
        }
    }
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }
    pub fn set_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }
    pub fn set_world_options(
        mut self,
        world_options: OxAgWorldGenerationOptions,
    ) -> Result<Self, OxAgError> {
        world_options.validate()?;
        self.world_options = Some(world_options);
        Ok(self)
    }
    pub fn set_content_options(
        mut self,
        mut content_options: HashMap<Content, OxAgContentOption>,
    ) -> Result<Self, OxAgError> {
        for content in Content::iter() {
            match &content {
                Content::None => {
                    let _ = content_options.remove(&content.to_default());
                }
                other => match content_options.get(&content.to_default()) {
                    Some(content_option) => {
                        content_option.validate(other)?;
                    }
                    None => {
                        content_options.insert(content.to_default(), Default::default());
                    }
                },
            }
        }
        self.content_options = Some(content_options);
        Ok(self)
    }
    pub fn set_world_options_from_preset(mut self, preset: OxAgWorldGenerationPresets) -> Self {
        self.world_options = Some(OxAgWorldGenerationOptions::from_preset(preset));
        self
    }
    pub fn set_content_options_from_preset(mut self, preset: OxAgContentGenerationPresets) -> Self {
        self.content_options = Some(OxAgContentOption::from_preset(preset));
        self
    }
    pub fn alter_content_gen_options(
        mut self,
        content: Content,
        content_option: OxAgContentOption,
    ) -> Result<Self, OxAgError> {
        content_option.validate(&content)?;
        if content == Content::None {
            Err(CannotSetContentOptionForNone)
        } else {
            let mut x = self.content_options;

            x.clone()
                .ok_or(ContentOptionNotSet(content.to_default()))?
                .entry(content.to_default())
                .and_modify(|value| *value = content_option)
                .or_insert(content_option.clone());

            self.content_options = x;
            Ok(self)
        }
    }
    pub fn set_environmental_conditions(
        mut self,
        weather_options: EnvironmentalConditions,
    ) -> Self {
        self.environmental_conditions = Some(weather_options);
        self
    }
    pub fn set_environmental_conditions_from_preset(
        mut self,
        preset: EnvironmentalConditionsPresets,
    ) -> Self {
        self.environmental_conditions = Some(EnvironmentalConditionsPresets::from_preset(preset));
        self
    }
}

impl OxAgWorldGenerator {
    pub fn get_size(&self) -> usize {
        self.size
    }
    pub fn get_seed(&self) -> u64 {
        self.seed
    }
    pub fn get_world_options(&self) -> &OxAgWorldGenerationOptions {
        &self.world_options
    }
    pub fn get_content_options(&self) -> &HashMap<Content, OxAgContentOption> {
        &self.content_options
    }
    pub fn get_weather_options(&self) -> &EnvironmentalConditions {
        &self.environmental_conditions
    }
    pub fn gen_map(&self) -> Vec<Vec<f64>> {
        // map init
        let mut map = vec![vec![0.0; self.size]; self.size];

        // perlin init
        let fbm_perlin = Fbm::<Perlin>::new(self.seed as u32)
            .set_octaves(DEFAULT_NOISE_OCTAVES)
            .set_frequency(DEFAULT_NOISE_FREQUENCY)
            .set_lacunarity(DEFAULT_NOISE_LACUNARITY)
            .set_persistence(DEFAULT_NOISE_PERSISTANCE);

        for y in 0..self.size {
            for x in 0..self.size {
                // initialization
                let (mut nx, mut ny) = (
                    x as f64 / self.size as f64 - 0.5,
                    y as f64 / self.size as f64 - 0.5,
                );

                // nx and ny frequency
                // let (nx_freq, ny_freq) = (2.5, 2.5);
                // nx *= nx_freq;
                // ny *= ny_freq;

                // map[y][x] = 1.0 * fbm_perlin.get([1.0 * nx, 1.0 * ny])
                //     + 0.5 * fbm_perlin.get([2.0 * nx, 2.0 * ny])
                //     + 0.25 * fbm_perlin.get([4.0 * nx, 4.0 * ny])
                //     + 0.125 * fbm_perlin.get([8.0 * nx, 8.0 * ny]);

                map[y][x] = fbm_perlin.get([nx, ny]);
            }
        }
        map
    }
}

impl Generator for OxAgWorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        let f64map = self.gen_map();
        todo!()
    }
}

use crate::utils::generate_random_seed;
use crate::utils::traits::FromSeed;
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use crate::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use crate::world_generator::OxAgWorldGenerator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::worldgenerator::Generator;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

impl OxAgWorldGenerator {
    pub fn save(&mut self, path: &str) -> io::Result<()> {
        let out = self.gen();
        File::create(path)?.write_all(
            serde_json::to_string(&(out.0, out.1, out.3, out.4))
                .unwrap()
                .as_bytes(),
        )
    }
}

impl OxAgWorldGeneratorBuilder {
    pub fn load(mut self, path: &str) -> Result<OxAgWorldGenerator, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let map_save: (
            Vec<Vec<Tile>>,
            (usize, usize),
            f32,
            Option<HashMap<Content, f32>>,
        ) = serde_json::from_str(&contents)?;

        Ok(OxAgWorldGenerator {
            size: map_save.0.len(),
            seed: 0,
            tile_type_options: OxAgTileTypeOptions {
                deep_water_level: 0.0..=0.0,
                shallow_water_level: 0.0..=0.0,
                sand_level: 0.0..=0.0,
                grass_level: 0.0..=0.0,
                hill_level: 0.0..=0.0,
                mountain_level: 0.0..=0.0,
                snow_level: 0.0..=0.0,
                river_n: 0..=0,
                street_n: 0..=0,
                street_len: 0..=0,
                lava_n: 0..=0,
                lava_radius: 0..=0,
            },
            map_save: Some(map_save),
            environmental_conditions: self.environmental_conditions.clone().unwrap_or(
                EnvironmentalConditions::new_from_seed(self.seed.unwrap_or(generate_random_seed())),
            ),
            height_multiplier: 0.0,
            score: 0.0,
            with_info: false,
            content_options: vec![],
            maze: false,
        })
    }
}

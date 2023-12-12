use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use crate::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use crate::world_generator::OxAgWorldGenerator;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{Read, Write};

impl OxAgWorldGenerator {
    pub fn save(&mut self, path: &str) -> io::Result<()> {
        File::create(path)?.write_all(serde_json::to_string(&self.gen()).unwrap().as_bytes())
    }
}

impl OxAgWorldGeneratorBuilder {
    pub fn load(self, path: &str) -> Result<OxAgWorldGenerator, Box<dyn std::error::Error>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let map_save: (
            Vec<Vec<Tile>>,
            (usize, usize),
            EnvironmentalConditions,
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
            environmental_conditions: EnvironmentalConditions::new(&[Sunny], 2, 2).unwrap(),
            map_save: Some(map_save),
            height_multiplier: 0.0,
            score: 0.0,
            with_info: false,
            content_options: vec![],
            maze: false,
            score_map: None,
        })
    }
}

pub mod constants;
pub mod environmental_condition_options;
pub mod tile_content_spawn_options;
pub mod tile_type_spawn_levels;
pub mod utilities;
pub mod world_generator_builder;
use crate::world_generator::constants::*;
use crate::world_generator::tile_content_spawn_options::OxAgTileContentSpawnOptions;
use crate::world_generator::tile_type_spawn_levels::OxAgTileTypeSpawnLevels;
use crate::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Content::Water;
use robotics_lib::world::tile::TileType::{DeepWater, Hill, Mountain, Sand, Snow};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::Generator;
use std::collections::HashMap;
use std::ops::{Not, RangeInclusive};

use super::world_generator::environmental_condition_options::OxAgEnvironmentalConditions;

/// World generator that implements the [Generator] trait.
///
/// To obtain this you should use a [OxAgWorldGeneratorBuilder].
///
/// # Example
/// ```rust
/// use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;
/// use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
///
/// let generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
///     .set_size(100)
///     .build();
/// ```
///
/// See [OxAgWorldGeneratorBuilder] documentation for cooler examples.
#[derive(Debug, Clone)]
pub struct OxAgWorldGenerator {
    /// Size that will be used to generate the world.
    ///
    /// Since the world is a square this indicates the width and height uniquely
    pub(crate) size: usize,

    /// Seed that the generator will use to generate the world.
    ///
    /// Two equal seeds will always produce the same exact base for the world.
    /// It can then differ if generated with different [world_options] and [content_options].
    pub(crate) seed: u64,

    /// Levels that will determine the spawn of the different tile types.
    pub(crate) tile_type_spawn_levels: OxAgTileTypeSpawnLevels,

    /// [HashMap] with the [Content] as the key and [OxAgTileContentSpawnOptions] as its value.
    pub(crate) tile_content_spawn_options: HashMap<Content, OxAgTileContentSpawnOptions>,

    /// [EnvironmentalConditions] that will be used in the generated world
    pub(crate) environmental_conditions: OxAgEnvironmentalConditions,

    /// [f64] height map multiplier
    pub(crate) height_multiplier: f64,
}

impl OxAgWorldGenerator {
    /// Returns a new builder.
    ///
    /// # Usage
    /// ```rust
    /// let generator = OxAgWorldGenerator::builder();
    /// // This is the same thing
    /// let another_generator = OxAgWorldGeneratorBuilder::new();
    /// ```
    pub fn builder() -> OxAgWorldGeneratorBuilder {
        OxAgWorldGeneratorBuilder::new()
    }

    /// Returns the size that will be used to generate the world.
    ///
    /// Since the world is a square the size indicates the width and height uniquely.
    pub fn get_size(&self) -> usize {
        self.size
    }

    /// Returns the seed that the generator will use to generate the world.
    ///
    /// Two equal seeds will always produce the same exact base for the world.
    /// It can then differ if generated with different [world_options] and [content_options].
    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    /// Returns the levels that will determine the spawn of the different tile types.
    pub fn get_tile_type_spawn_levels(&self) -> &OxAgTileTypeSpawnLevels {
        &self.tile_type_spawn_levels
    }

    /// Returns an [HashMap] with the [Content] as the key and [OxAgTileContentSpawnOptions] as its value
    pub fn get_tile_content_spawn_options(&self) -> &HashMap<Content, OxAgTileContentSpawnOptions> {
        &self.tile_content_spawn_options
    }

    /// Returns the [EnvironmentalConditions] that will be used in the generated world
    pub fn get_environmental_conditions(&self) -> &OxAgEnvironmentalConditions {
        &self.environmental_conditions
    }

    /// Returns matrix of floats generated from the seed.
    ///
    /// This float values are meant to be mapped to tile types considering the tile type spawn levels.
    pub fn generate_float_matrix(&self) -> (Vec<Vec<f64>>, f64, f64) {
        let rng = StdRng::seed_from_u64(self.seed);
        // map init
        let mut map = vec![vec![0.0; self.size]; self.size];

        // perlin init
        let fbm_perlin = Fbm::<Perlin>::new(self.seed as u32)
            .set_octaves(DEFAULT_NOISE_OCTAVES)
            .set_frequency(DEFAULT_NOISE_FREQUENCY)
            .set_lacunarity(DEFAULT_NOISE_LACUNARITY)
            .set_persistence(DEFAULT_NOISE_PERSISTANCE);

        let mut min = f64::MAX;
        let mut max = f64::MIN;

        let mut set_flow = false;

        map.iter_mut().enumerate().for_each(|(y, row)| {
            row.iter_mut().enumerate().for_each(|(x, (cell))| {
                let (nx, ny) = (x as f64 / self.size as f64, y as f64 / self.size as f64);
                *cell = fbm_perlin.get([nx, ny]);
                if *cell < min {
                    min = *cell;
                }
                if *cell > max {
                    max = *cell;
                }
            });
        });
        (map, min, max)
    }

    /// Returns matrix of [Tile] conforming to the generator configuration
    fn generate_tile_matrix(
        &self,
        float_matrix: &Vec<Vec<f64>>,
        min: f64,
        max: f64,
    ) -> Vec<Vec<Tile>> {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut tile_map = vec![
            vec![
                Tile {
                    tile_type: TileType::Grass,
                    content: Content::None,
                    elevation: 0,
                };
                self.size
            ];
            self.size
        ];

        float_matrix.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &value)| {
                let normalized_value = if value > 0.0 {
                    value / max
                } else {
                    -value / min
                };
                let level = &self.tile_type_spawn_levels;
                match normalized_value {
                    value if level.deep_water_level.contains(&value) => {
                        tile_map[i][j] = Tile {
                            tile_type: TileType::DeepWater,
                            content: Content::Water(
                                rng.gen_range(0.0..Water(0).properties().max() as f64) as usize,
                            ),
                            elevation: ((value + 1.0) * self.height_multiplier) as usize,
                        }
                    }
                    value if level.shallow_water_level.contains(&value) => {
                        tile_map[i][j] = Tile {
                            tile_type: TileType::ShallowWater,
                            content: Content::Water(
                                rng.gen_range(0.0..Water(0).properties().max() as f64) as usize,
                            ),
                            elevation: ((value + 1.0) * self.height_multiplier) as usize,
                        }
                    }
                    value if level.sand_level.contains(&value) => {
                        tile_map[i][j].tile_type = Sand;
                    }
                    value if level.hill_level.contains(&value) => {
                        tile_map[i][j].tile_type = Hill;
                    }
                    value if level.mountain_level.contains(&value) => {
                        tile_map[i][j].tile_type = Mountain;
                    }
                    value if level.snow_level.contains(&value) => {
                        tile_map[i][j].tile_type = Snow;
                    }
                    _ => {}
                }
            })
        });

        self.spawn_contents(&mut tile_map);

        tile_map
    }

    fn spawn_contents(&self, tile_map: &mut Vec<Vec<Tile>>) {
        tile_map
            .iter_mut()
            .enumerate()
            .for_each(|(i, row)| row.iter_mut().enumerate().for_each(|(j, cell)| {}))
    }
}

impl Generator for OxAgWorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32) {
        let (map, min, max) = self.generate_float_matrix();
        (
            self.generate_tile_matrix(&map, min, max),
            (self.size, self.size),
            self.environmental_conditions.clone().into(),
            0.0,
        )
    }
}

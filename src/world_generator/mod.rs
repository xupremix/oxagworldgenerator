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
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::worldgenerator::Generator;
use std::collections::HashMap;

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
///     .set_size(Some(100))
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
    pub fn generate_float_matrix(&self) -> Vec<Vec<f64>> {
        // map init
        let mut map = vec![vec![0.0; self.size]; self.size];

        // perlin init
        let fbm_perlin = Fbm::<Perlin>::new(self.seed as u32)
            .set_octaves(DEFAULT_NOISE_OCTAVES)
            .set_frequency(DEFAULT_NOISE_FREQUENCY)
            .set_lacunarity(DEFAULT_NOISE_LACUNARITY)
            .set_persistence(DEFAULT_NOISE_PERSISTANCE);

        for (y, row) in map.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                // initialization
                let (nx, ny) = (
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

                *cell = fbm_perlin.get([nx, ny]);
            }
        }
        map
    }

    /// Returns matrix of [Tile] conforming to the generator configuration
    fn generate_tile_matrix(&self) -> Vec<Vec<Tile>> {
        let _float_matrix = self.generate_float_matrix();
        todo!("Map float matrix to tile types based on levels");
    }
}

impl Generator for OxAgWorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions) {
        (
            self.generate_tile_matrix(),
            (self.size, self.size),
            self.environmental_conditions.clone().into(),
        )
    }
}

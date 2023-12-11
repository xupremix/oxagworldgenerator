use std::collections::HashMap;

use crate::utils::errors::OxAgError;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::{Content, Tile};
use robotics_lib::world::world_generator::Generator;

use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::maze::maze_builder_init;
use crate::world_generator::spawning_tools::{matrix_spawn::f64_mat, F64MatData, MazeBuilder};
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use crate::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;

pub mod content_options;
pub mod environmental_condition_options;
pub mod presets;
mod serial;
mod spawning_tools;
pub mod tile_type_options;
pub mod world_generator_builder;

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
    pub(crate) tile_type_options: OxAgTileTypeOptions,

    /// [HashMap] with the [Content] as the key and [OxAgContentOptions] as its value.
    pub(crate) content_options: Vec<(Content, OxAgContentOptions)>,

    /// [EnvironmentalConditions] that will be used in the generated world
    pub(crate) environmental_conditions: EnvironmentalConditions,

    /// [f64] height map multiplier
    pub(crate) height_multiplier: f64,

    /// [f32] score
    pub(crate) score: f32,

    /// [bool] with_info
    pub(crate) with_info: bool,

    pub(crate) maze: bool,

    pub(crate) score_map: Option<HashMap<Content, f32>>,

    pub(crate) map_save: Option<(
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    )>,
}

impl OxAgWorldGenerator {
    /// Returns a new builder.
    ///
    /// # Usage
    /// ```rust
    /// use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;
    /// use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
    ///
    /// let generator = OxAgWorldGenerator::builder();
    /// // This is the same thing
    /// let another_generator = OxAgWorldGeneratorBuilder::new().build();
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

    pub fn get_score(&self) -> f32 {
        self.score
    }
    pub fn get_maze(&self) -> bool {
        self.maze
    }

    pub fn get_with_info(&self) -> bool {
        self.with_info
    }

    /// Returns the seed that the generator will use to generate the world.
    ///
    /// Two equal seeds will always produce the same exact base for the world.
    /// It can then differ if generated with different [world_options] and [content_options].
    pub fn get_seed(&self) -> u64 {
        self.seed
    }

    /// Returns the levels that will determine the spawn of the different tile types.
    pub fn get_tile_type_options(&self) -> &OxAgTileTypeOptions {
        &self.tile_type_options
    }

    /// Returns an [HashMap] with the [Content] as the key and [OxAgContentOptions] as its value
    pub fn get_content_options(&self) -> &Vec<(Content, OxAgContentOptions)> {
        &self.content_options
    }

    /// Returns the [EnvironmentalConditions] that will be used in the generated world
    pub fn get_environmental_conditions(&self) -> &EnvironmentalConditions {
        &self.environmental_conditions
    }

    pub fn get_score_map(&self) -> &Option<HashMap<Content, f32>> {
        &self.score_map
    }

    /// Returns matrix of floats generated from the seed.
    ///
    /// This float values are meant to be mapped to tile types considering the tile type spawn levels.
    fn generate_float_matrix(&self) -> F64MatData {
        f64_mat(self.seed, self.size, self.with_info)
    }

    /// Returns a matrix filled with wall.
    ///
    /// This matrix will become a maze.
    fn generate_base_maze(&mut self) -> MazeBuilder {
        if (self.size % 2 == 0) {
            self.size += 1;
        }
        maze_builder_init(self.seed, self.size)
    }
}

impl Generator for OxAgWorldGenerator {
    fn gen(
        &mut self,
    ) -> (
        Vec<Vec<Tile>>,
        (usize, usize),
        EnvironmentalConditions,
        f32,
        Option<HashMap<Content, f32>>,
    ) {
        if self.map_save.is_some() {
            return self.map_save.clone().unwrap();
        }
        if self.maze {
            let mut map = self.generate_base_maze().builder();
            (
                map.0,
                map.1,
                self.environmental_conditions.clone(),
                self.score,
                self.score_map.clone(),
            )
        } else {
            let (map, spawn) = self
                .generate_float_matrix()
                .to_tile_mat(self.get_tile_type_options(), self.height_multiplier)
                .spawn_contents(self.get_content_options());
            (
                map.map,
                spawn,
                self.environmental_conditions.clone(),
                self.score,
                self.score_map.clone(),
            )
        }
    }
}

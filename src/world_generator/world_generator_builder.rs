use std::collections::HashMap;

use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Content;

use crate::utils::constants::DEFAULT_SCORE;
use crate::utils::errors::OxAgError;
use crate::utils::errors::OxAgError::ContentOptionsNotSet;
use crate::utils::traits::Loadable;
use crate::utils::traits::{FromSeed, Validator};
use crate::utils::{generate_random_seed, generate_random_world_size, multiplier_from_seed};
use crate::world_generator::presets::content_presets::OxAgContentPresets;
use crate::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use crate::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use crate::world_generator::OxAgWorldGenerator;

use super::content_options::OxAgContentOptions;

/// World generator builder that can be used to obtain a [OxAgWorldGenerator].
///
/// # Usage
/// This builder allows to set the following options for the [OxAgWorldGenerator]:
/// * `size` - size that will be used to generate the world
/// * `seed` - Seed that the [OxAgWorldGenerator] will use to generate the world.
/// * `tile_type_spawn_levels` - levels that will determine the spawn of the different tile types.
/// * `tile_content_spawn_options` - [HashMap] with the [Content] as the key and [OxAgContentOptions] as its value.
/// * `environmental_conditions` - [EnvironmentalConditions] that will be used in the generated world.
///
/// All those properties are [Option], and by default they are set to [None].
///
/// Any property set to [None] will be calculated via the seed.
/// The calculation is not random, a seed always produces the same properties.
///
/// (If the seed is also [None] it will be randomly generated].
///
/// # Example
/// ```rust
/// use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
///
/// let default_generator = OxAgWorldGeneratorBuilder::new().build();
///
/// let custom_size_generator = OxAgWorldGeneratorBuilder::new()
///     .set_size(100)
///     .build().unwrap();
///
/// ```
pub struct OxAgWorldGeneratorBuilder {
    /// Optional size that will be used to generate the world.
    ///
    /// If [None] it will be calculated via the seed.
    pub(crate) size: Option<usize>,

    /// Seed that the [OxAgWorldGenerator] will use to generate the world.
    ///
    /// Two equal seeds will always produce the same exact base for the world.
    /// It can then differ if generated with different [world_options] and [content_options].
    ///
    /// This is optional, if [None] a random seed will be generated.
    pub(crate) seed: Option<u64>,

    /// Optional levels that will determine the spawn of the different tile types.
    ///
    /// If [None] they will be calculated via the seed.
    pub(crate) tile_type_options: Option<OxAgTileTypeOptions>,

    /// Optional [HashMap] with the [Content] as the key and [OxAgContentOptions] as its value.
    ///
    /// If [None] it will be calculated via the seed.
    pub(crate) content_options: Option<Vec<(Content, OxAgContentOptions)>>,

    /// Optional [OxAgEnvironmentalConditions] that will be used in the generated world.
    ///
    /// If [None] they will be calculated via the seed.
    pub(crate) environmental_conditions: Option<EnvironmentalConditions>,

    /// Optional [f64] that will be used to calculate the height of the map.
    ///
    /// If [None] they will be calculated via the seed.
    pub(crate) height_multiplier: Option<f64>,

    /// Optional [f32] that will be used to set the score
    ///
    /// If [None] they will be calculated via the seed.
    pub(crate) score: Option<f32>,

    pub(crate) maze: Option<bool>,

    /// Optional [bool] that is used to log the actions to console
    pub(crate) with_info: Option<bool>,

    pub(crate) score_map: Option<HashMap<Content, f32>>,
}

impl OxAgWorldGeneratorBuilder {
    /// Builds the [OxAgWorldGenerator] using its options:
    /// * `size` - size that will be used to generate the world
    /// * `seed` - Seed that the [OxAgWorldGenerator] will use to generate the world.
    /// * `tile_type_options` - levels that will determine the spawn of the different tile types.
    /// * `content_options` - [HashMap] with the [Content] as the key and [OxAgContentOptions] as its value.
    /// * `environmental_conditions` - [EnvironmentalConditions] that will be used in the generated world.
    ///
    /// All those properties are [Option], and by default they are set to [None].
    ///
    /// Any property set to [None] will be calculated via the seed.
    /// The calculation is not random, a seed always produces the same properties.
    ///
    /// (If the seed is also [None] it will be randomly generated].
    ///
    /// Returns the WorldGenerator [Generator](OxAgWorldGenerator)
    ///
    /// # Examples
    /// ```rust
    /// use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
    /// let generator = OxAgWorldGeneratorBuilder::new().build();
    /// ```
    pub fn build(&self) -> Result<OxAgWorldGenerator, OxAgError> {
        let seed = self.seed.unwrap_or(generate_random_seed());
        let size = self.size.unwrap_or(generate_random_world_size(seed));

        if size < 5 && self.maze.unwrap_or(false) {
            return Err(OxAgError::MazeSizeTooSmall);
        }

        Ok(OxAgWorldGenerator {
            size,
            seed,
            tile_type_options: self
                .tile_type_options
                .clone()
                .unwrap_or(OxAgTileTypeOptions::new_from_seed(seed)),
            content_options: self
                .content_options
                .clone()
                .unwrap_or(OxAgContentOptions::new_from_seed(seed, size)),
            environmental_conditions: self
                .environmental_conditions
                .clone()
                .unwrap_or(EnvironmentalConditions::new_from_seed(seed)),
            height_multiplier: self.height_multiplier.unwrap_or(multiplier_from_seed(seed)),
            score: self.score.unwrap_or(DEFAULT_SCORE),
            with_info: self.with_info.unwrap_or(true),
            maze: self.maze.unwrap_or(false),
            score_map: self.score_map.clone(),
            map_save: None,
        })
    }

    /// Returns the [Builder](OxAgWorldGeneratorBuilder) with the properties not set.
    pub fn new() -> Self {
        Self {
            size: None,
            seed: None,
            tile_type_options: None,
            content_options: None,
            environmental_conditions: None,
            height_multiplier: None,
            score: None,
            maze: None,
            with_info: None,
            score_map: None,
        }
    }

    /// Sets the seed of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Sets the score of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)\
    pub fn set_score(mut self, score: f32) -> Self {
        self.score = Some(score);
        self
    }

    /// Sets the score of the Map
    ///
    /// Return the Score Map
    /// TODO: Actually writing what this does...
    pub fn set_score_map(mut self, score_map: Option<HashMap<Content, f32>>) -> Self {
        self.score_map = score_map;
        self
    }

    pub fn set_maze(mut self, maze: bool) -> Self {
        self.maze = Some(maze);
        self
    }

    ///  Sets the with_info of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_with_info(mut self, with_info: bool) -> Self {
        self.with_info = Some(with_info);
        self
    }

    /// Sets the height multiplier of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_height_multiplier(mut self, multiplier: f64) -> Self {
        self.height_multiplier = Some(multiplier);
        self
    }

    /// Sets the size of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_size(mut self, size: usize) -> Self {
        self.size = Some(size);
        self
    }

    /// Sets the tile type spawn levels of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns a [Result] of the [Builder](OxAgWorldGeneratorBuilder) or an [OxAgError] if the options are invalid.
    pub fn set_tile_type_options(
        mut self,
        tile_type_options: OxAgTileTypeOptions,
    ) -> Result<Self, OxAgError> {
        tile_type_options.validate()?;
        self.tile_type_options = Some(tile_type_options);
        Ok(self)
    }

    /// Sets the tile content spawn options of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_content_options(
        mut self,
        content_options: Vec<(Content, OxAgContentOptions)>,
    ) -> Result<Self, OxAgError> {
        content_options.validate()?;
        self.content_options = Some(content_options);
        Ok(self)
    }

    /// Sets the [EnvironmentalConditions] of the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_environmental_conditions(
        mut self,
        environmental_conditions: EnvironmentalConditions,
    ) -> Self {
        self.environmental_conditions = Some(environmental_conditions);
        self
    }

    /// Sets the tile type spawn levels from a [OxAgWorldGenerationPresets] preset.
    pub fn set_tile_type_options_from_preset(mut self, preset: OxAgTileTypePresets) -> Self {
        self.tile_type_options = Some(preset.load());
        self
    }

    /// Sets the tile content spawn options from a [OxAgContentGenerationPresets] preset.
    pub fn set_content_options_from_preset(mut self, preset: OxAgContentPresets) -> Self {
        self.content_options = Some(preset.load());
        self
    }

    /// Sets the [EnvironmentalConditions] from a [EnvironmentalConditionsPresets] preset.
    pub fn set_environmental_conditions_from_preset(
        mut self,
        preset: OxAgEnvironmentalConditionPresets,
    ) -> Self {
        self.environmental_conditions = Some(preset.load());
        self
    }

    /// Modifies a single tile content spawn options.
    /// This will also perform a check to validate the provided options.
    ///
    /// Returns a [Result] of the [Builder](OxAgWorldGeneratorBuilder) or an [OxAgError] if the options are invalid.
    pub fn alter_content_option(
        mut self,
        content: Content,
        content_option: OxAgContentOptions,
    ) -> Result<Self, OxAgError> {
        content_option.validate()?;
        let found = self
            .content_options
            .as_mut()
            .ok_or(ContentOptionsNotSet)?
            .iter_mut()
            .any(|(c, opt)| {
                if *c == content.to_default() {
                    *opt = content_option;
                    true
                } else {
                    false
                }
            });
        if !found {
            let mut option = self.content_options.clone().unwrap();
            option.push((content.to_default(), content_option));
            self.content_options = Some(option);
        }
        Ok(self)
    }
}

impl Default for OxAgWorldGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

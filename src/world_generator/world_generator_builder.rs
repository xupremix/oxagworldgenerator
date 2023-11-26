use std::collections::HashMap;

use robotics_lib::world::tile::Content;
use strum::IntoEnumIterator;

use crate::utils::errors::OxAgError;
use crate::world_generator::tile_type_spawn_levels::OxAgTileTypeSpawnLevels;
use crate::world_generator::utilities::{generate_random_seed, generate_random_world_size};
use crate::world_generator::world_generator::OxAgWorldGenerator;

use super::environmental_condition_options::{
    OxAgEnvironmentalConditions, OxAgEnvironmentalConditionsPresets,
};
use super::tile_content_spawn_options::{
    OxAgTileContentSpawnOptionPresets, OxAgTileContentSpawnOptions,
};
use super::tile_type_spawn_levels::OxAgTileTypeSpawnLevelPresets;

/// World generator builder that can be used to obtain a [OxAgWorldGenerator].
///
/// # Usage
/// This builder allows to set the following options for the [OxAgWorldGenerator]:
/// * `size` - size that will be used to generate the world
/// * `seed` - Seed that the [OxAgWorldGenerator] will use to generate the world.
/// * `tile_type_spawn_levels` - levels that will determine the spawn of the different tile types.
/// * `tile_content_spawn_options` - [HashMap] with the [Content] as the key and [OxAgTileContentSpawnOptions] as its value.
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
/// use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
///
/// let defaultGenerator = OxAgWorldGeneratorBuilder::new().build();
///
/// let customSizeGenerator = OxAgWorldGeneratorBuilder::new()
///     .set_size(Some(100))
///     .build();
///
/// TODO("Other examples")
/// ```
pub struct OxAgWorldGeneratorBuilder {
    /// Optional size that will be used to generate the world.
    ///
    /// If [None] it will be calculated via the seed.
    size: Option<usize>,

    /// Seed that the [OxAgWorldGenerator] will use to generate the world.
    ///
    /// Two equal seeds will always produce the same exact base for the world.
    /// It can then differ if generated with different [world_options] and [content_options].
    ///
    /// This is optional, if [None] a random seed will be generated.
    seed: Option<u64>,

    /// Optional levels that will determine the spawn of the different tile types.
    ///
    /// If [None] they will be calculated via the seed.
    tile_type_spawn_levels: Option<OxAgTileTypeSpawnLevels>,

    /// Optional [HashMap] with the [Content] as the key and [OxAgTileContentSpawnOptions] as its value.
    ///
    /// If [None] it will be calculated via the seed.
    tile_content_spawn_options: Option<HashMap<Content, OxAgTileContentSpawnOptions>>,

    /// Optional [OxAgEnvironmentalConditions] that will be used in the generated world.
    ///
    /// If [None] they will be calculated via the seed.
    environmental_conditions: Option<OxAgEnvironmentalConditions>,
}

impl OxAgWorldGeneratorBuilder {
    /// Builds the [OxAgWorldGenerator] using its options:
    /// * `size` - size that will be used to generate the world
    /// * `seed` - Seed that the [OxAgWorldGenerator] will use to generate the world.
    /// * `tile_type_spawn_levels` - levels that will determine the spawn of the different tile types.
    /// * `tile_content_spawn_options` - [HashMap] with the [Content] as the key and [OxAgTileContentSpawnOptions] as its value.
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
    /// use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
    /// let generator = OxAgWorldGeneratorBuilder::new().build();
    /// ```
    pub fn build(&self) -> OxAgWorldGenerator {
        let seed = self.seed.unwrap_or(generate_random_seed());

        OxAgWorldGenerator {
            size: self.size.unwrap_or(generate_random_world_size(seed)),
            seed,
            tile_type_spawn_levels: self
                .tile_type_spawn_levels
                .clone()
                .unwrap_or(OxAgTileTypeSpawnLevels::new_from_seed(seed)),
            tile_content_spawn_options: self
                .tile_content_spawn_options
                .clone()
                .unwrap_or(OxAgTileContentSpawnOptions::new_from_seed(seed)),
            environmental_conditions: self
                .environmental_conditions
                .clone()
                .unwrap_or(OxAgEnvironmentalConditions::new_from_seed(seed)),
        }
    }

    /// Returns the [Builder](OxAgWorldGeneratorBuilder) with the properties not set.
    pub fn new() -> Self {
        Self {
            size: None,
            seed: None,
            tile_type_spawn_levels: None,
            tile_content_spawn_options: None,
            environmental_conditions: None,
        }
    }

    /// Sets the seed of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
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
    pub fn set_tile_type_spawn_levels(
        mut self,
        tile_type_spawn_levels: OxAgTileTypeSpawnLevels,
    ) -> Result<Self, OxAgError> {
        tile_type_spawn_levels.validate();
        self.tile_type_spawn_levels = Some(tile_type_spawn_levels);
        Ok(self)
    }

    /// Sets the tile content spawn options of the [Builder](OxAgWorldGeneratorBuilder)
    ///
    /// Returns a [Result] of the [Builder](OxAgWorldGeneratorBuilder) or an [OxAgError] if the options are invalid.
    pub fn set_tile_content_spawn_options(
        mut self,
        mut tile_content_spawn_options: HashMap<Content, OxAgTileContentSpawnOptions>,
    ) -> Result<Self, OxAgError> {
        for content in Content::iter() {
            match &content {
                Content::None => {
                    tile_content_spawn_options.remove(&content.to_default());
                }
                other => match tile_content_spawn_options.get(&content.to_default()) {
                    Some(content_option) => {
                        content_option.validate(other)?;
                    }
                    None => {
                        tile_content_spawn_options.insert(content.to_default(), Default::default());
                    }
                },
            }
        }
        self.tile_content_spawn_options = Some(tile_content_spawn_options);
        Ok(self)
    }

    /// Sets the tile type spawn levels from a [OxAgWorldGenerationPresets] preset.
    pub fn set_tile_type_spawn_levels_from_preset(
        mut self,
        preset: OxAgTileTypeSpawnLevelPresets,
    ) -> Self {
        self.tile_type_spawn_levels = Some(OxAgTileTypeSpawnLevels::from_preset(preset));
        self
    }

    /// Sets the tile content spawn options from a [OxAgContentGenerationPresets] preset.
    pub fn set_tile_content_spawn_options_from_preset(
        mut self,
        preset: OxAgTileContentSpawnOptionPresets,
    ) -> Self {
        self.tile_content_spawn_options =
            Some(OxAgTileContentSpawnOptions::new_from_preset(preset));
        self
    }

    /// Modifies a single tile content spawn options.
    /// This will also perform a check to validate the provided options.
    ///
    /// Returns a [Result] of the [Builder](OxAgWorldGeneratorBuilder) or an [OxAgError] if the options are invalid.
    pub fn alter_content_gen_options(
        mut self,
        content: Content,
        content_option: OxAgTileContentSpawnOptions,
    ) -> Result<Self, OxAgError> {
        content_option.validate(&content)?;

        if content == Content::None {
            Err(OxAgError::CannotSetContentOptionForNone)
        } else {
            let options = self
                .tile_content_spawn_options
                .as_mut()
                .ok_or(OxAgError::ContentOptionNotSet(content.to_default()))?;

            match options.get_mut(&content.to_default()) {
                Some(value) => {
                    *value = content_option;
                }
                None => {
                    options.insert(content.to_default(), content_option);
                }
            }

            Ok(self)
        }
    }

    /// Sets the [EnvironmentalConditions] of the [Builder](OxAgWorldGeneratorBuilder)
    pub fn set_environmental_conditions(
        mut self,
        environmental_conditions: OxAgEnvironmentalConditions,
    ) -> Self {
        self.environmental_conditions = Some(environmental_conditions);
        self
    }

    /// Sets the [EnvironmentalConditions] from a [EnvironmentalConditionsPresets] preset.
    pub fn set_environmental_conditions_from_preset(
        mut self,
        preset: OxAgEnvironmentalConditionsPresets,
    ) -> Self {
        self.environmental_conditions = Some(OxAgEnvironmentalConditions::new_from_preset(preset));
        self
    }
}

impl Default for OxAgWorldGeneratorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

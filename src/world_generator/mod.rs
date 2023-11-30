pub mod constants;
pub mod environmental_condition_options;
pub mod tile_content_spawn_options;
pub mod tile_type_spawn_levels;
pub mod utilities;
pub mod world_generator_builder;

use crate::world_generator::constants::*;
use crate::world_generator::tile_content_spawn_options::OxAgTileContentSpawnOptions;
use crate::world_generator::tile_type_spawn_levels::OxAgTileTypeSpawnLevels;
use crate::world_generator::utilities::ToValue;
use crate::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::tile::Content::{Fire, Fish, Water};
use robotics_lib::world::tile::TileType::{DeepWater, Hill, Mountain, Sand, Snow};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::{get_tiletype_percentage, Generator};
use std::cmp::min;
use std::collections::HashMap;
use std::ops::{Not, Range, RangeInclusive};
use strum::IntoEnumIterator;

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

    /// [f32] score
    pub(crate) score: f32,
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

    pub fn get_score(&self) -> f32 {
        self.score
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
                    _ => {
                        // distance from the nearest bound
                    }
                }
            })
        });

        self.spawn_contents(&mut tile_map);

        tile_map
    }

    fn spawn_contents(&self, tile_map: &mut Vec<Vec<Tile>>) {
        let percentage_map = get_tiletype_percentage(tile_map);

        // let contents = self
        //     .tile_content_spawn_options
        //     .iter()
        //     .collect::<Vec<(&Content, &OxAgTileContentSpawnOptions)>>()
        //     .sort();
        for (content, content_option) in self.tile_content_spawn_options.iter() {
            let content = &content.to_default();
            if content_option.is_present {
                let percentage = TileType::iter()
                    .filter_map(|tiletype| {
                        if tiletype.properties().can_hold(content) {
                            match percentage_map.get(&tiletype) {
                                Some(percentage) => Some(percentage),
                                None => Some(&0.0),
                            }
                        } else {
                            None
                        }
                    })
                    .sum::<f64>();
                if content_option.in_batches {
                    println!("Adding {:?} in batches", content);
                    self.spawn_in_batches(content, content_option, tile_map, percentage);
                } else {
                    println!("Adding {:?} ", content);
                    self.spawn_randomly(content, content_option, tile_map, percentage);
                }
            } else {
                println!("Skipping {:?}", content);
            }
        }
    }

    fn spawn_in_batches(
        &self,
        content: &Content,
        content_option: &OxAgTileContentSpawnOptions,
        tile_map: &mut Vec<Vec<Tile>>,
        percentage: f64,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut radius = 1.0;
        if content_option.max_radius != 0 {
            radius = rng.gen_range(1.0..content_option.max_radius as f64);
        }
        let max_spawn_number = if content_option.with_max_spawn_number {
            content_option.max_spawn_number
        } else {
            rng.gen_range(
                content_option.min_spawn_number
                    ..((self.size.pow(2) as f64 * percentage)
                        / (radius.powi(2) * 3.14 + DEFAULT_BATCH_DISTANCE as f64))
                        as usize,
            )
        };
        for _ in 0..max_spawn_number {
            let mut row = 0;
            let mut col = 0;
            loop {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
                if tile_map[row][col].tile_type.properties().can_hold(content) {
                    break;
                }
            }
            self.spawn_circle(tile_map, row, col, radius as usize, content);
        }
    }

    fn spawn_circle(
        &self,
        matrix: &mut Vec<Vec<Tile>>,
        center_x: usize,
        center_y: usize,
        radius: usize,
        content: &Content,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let matrix_size = matrix.len();
        let min_radius = radius.min(
            center_x
                .min(center_y)
                .min(matrix_size - center_x - 1)
                .min(matrix_size - center_y - 1),
        ) as isize;

        let mut x: isize = min_radius;
        let mut y: isize = 0;
        let mut decision = 1 - x; // Decision parameter to determine next point

        let mut value = 0;
        if content.properties().max() != 0 {
            value = rng.gen_range(0..content.properties().max());
        }

        let center_x = center_x as isize;
        let center_y = center_y as isize;
        while x >= y {
            // Plot points using symmetry in all octants
            self.add(matrix, center_x + x, center_y + y, content);
            self.add(matrix, center_x + y, center_y + x, content);
            self.add(matrix, center_x - y, center_y + x, content);
            self.add(matrix, center_x - x, center_y + y, content);
            self.add(matrix, center_x - x, center_y - y, content);
            self.add(matrix, center_x - y, center_y - x, content);
            self.add(matrix, center_x + y, center_y - x, content);
            self.add(matrix, center_x + x, center_y - y, content);

            y += 1;
            if decision <= 0 {
                decision += 2 * y + 1;
            } else {
                x -= 1;
                decision += 2 * (y - x) + 1;
            }
        }

        // Fill the center of the circle
        for i in center_x - min_radius + 1..center_x + min_radius {
            for j in center_y - min_radius + 1..center_y + min_radius {
                if (i - center_x).pow(2) + (j - center_y).pow(2) <= min_radius.pow(2) as isize {
                    self.add(matrix, i, j, content);
                }
            }
        }
    }

    fn add(&self, map: &mut Vec<Vec<Tile>>, row: isize, col: isize, content: &Content) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut value = 0;
        if content.properties().max() != 0 {
            value = rng.gen_range(0..content.properties().max());
        }
        let row = row as usize;
        let col = col as usize;
        if map[row][col].tile_type.properties().can_hold(content) {
            map[row][col].content = content.to(value);
        }
    }

    fn spawn_randomly(
        &self,
        content: &Content,
        content_option: &OxAgTileContentSpawnOptions,
        tile_map: &mut Vec<Vec<Tile>>,
        percentage: f64,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let max_spawn_number = if content_option.with_max_spawn_number {
            content_option.max_spawn_number
        } else {
            rng.gen_range(
                content_option.min_spawn_number..(self.size.pow(2) as f64 * percentage) as usize,
            )
        };
        for _ in 0..max_spawn_number {
            let mut row = 0;
            let mut col = 0;
            loop {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
                if tile_map[row][col].tile_type.properties().can_hold(content) {
                    break;
                }
            }
            let mut value = 0;
            if content.properties().max() != 0 {
                value = rng.gen_range(0..content.properties().max());
            }
            tile_map[row][col].content = content.to(value);
        }
    }
}

impl Generator for OxAgWorldGenerator {
    fn gen(&mut self) -> (Vec<Vec<Tile>>, (usize, usize), EnvironmentalConditions, f32) {
        let (map, min, max) = self.generate_float_matrix();
        (
            self.generate_tile_matrix(&map, min, max),
            (self.size, self.size),
            self.environmental_conditions.clone().into(),
            self.score,
        )
    }
}

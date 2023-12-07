use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content::Water;
use robotics_lib::world::tile::TileType::{DeepWater, Hill, Mountain, Sand, ShallowWater, Snow};
use robotics_lib::world::tile::{Content, Tile, TileType};

use crate::utils::constants::{
    DEFAULT_NOISE_FREQUENCY, DEFAULT_NOISE_LACUNARITY, DEFAULT_NOISE_OCTAVES,
    DEFAULT_NOISE_PERSISTENCE,
};
use crate::utils::progress_bar;
use crate::world_generator::spawning_tools::{F64MatData, TileMat};
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;

pub(crate) fn f64_mat(seed: u64, size: usize, with_info: bool) -> F64MatData {
    // map init
    let mut map = vec![vec![0.0; size]; size];

    // perlin init
    let fbm_perlin = Fbm::<Perlin>::new(seed as u32)
        .set_octaves(DEFAULT_NOISE_OCTAVES)
        .set_frequency(DEFAULT_NOISE_FREQUENCY)
        .set_lacunarity(DEFAULT_NOISE_LACUNARITY)
        .set_persistence(DEFAULT_NOISE_PERSISTENCE);

    let mut min = f64::MAX;
    let mut max = f64::MIN;

    let mut i = (0, size.pow(2));
    map.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut().enumerate().for_each(|(x, cell)| {
            let (nx, ny) = (x as f64 / size as f64, y as f64 / size as f64);
            *cell = fbm_perlin.get([nx, ny]);
            if *cell < min {
                min = *cell;
            }
            if *cell > max {
                max = *cell;
            }
            if with_info {
                progress_bar(i.0, i.1, "Generating height map:", 50, "■");
                i.0 += 1;
            }
        });
    });
    F64MatData {
        map,
        min,
        max,
        seed,
        size,
        with_info,
    }
}

impl F64MatData {
    pub(crate) fn to_tile_mat(
        self,
        spawn_levels: &OxAgTileTypeOptions,
        multiplier: f64,
    ) -> TileMat {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut map = vec![
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

        let mut idx = (0, self.size.pow(2));
        self.map.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, &value)| {
                let normalized_value = self.normalizer(value);
                if (map[i][j].tile_type != ShallowWater) {
                    match normalized_value {
                        value
                        if spawn_levels.deep_water_level.contains(&value)
                            | spawn_levels.shallow_water_level.contains(&value) =>
                            {
                                let tile_type = if spawn_levels.deep_water_level.contains(&value) {
                                    DeepWater
                                } else {
                                    ShallowWater
                                };
                                let content =
                                    Water(rng.gen_range(0.0..Water(0).properties().max() as f64) as usize);
                                let elevation = ((value + 1.0) * multiplier) as usize;
                                map[i][j] = Tile {
                                    tile_type,
                                    content,
                                    elevation,
                                }
                            }
                        value if spawn_levels.sand_level.contains(&value) => {
                            map[i][j].tile_type = Sand;
                        }
                        value if spawn_levels.hill_level.contains(&value) => {
                            map[i][j].tile_type = Hill;
                        }
                        value if spawn_levels.mountain_level.contains(&value) => {
                            if rng.gen_bool(0.30) {
                                self.river_builder(&mut map, i, j, crate::world_generator::spawning_tools::matrix_spawn::RiverDirection::None);
                            } else {
                                map[i][j].tile_type = Mountain;
                            }
                        }
                        value if spawn_levels.snow_level.contains(&value) => {
                            if rng.gen_bool(0.30) {
                                self.river_builder(&mut map, i, j, RiverDirection::None);
                            } else {
                                map[i][j].tile_type = Snow;
                            }
                        }
                        _ => {
                            // distance from the nearest bound
                        }
                    }
                }
                if self.with_info {
                    progress_bar(idx.0, idx.1, "Generating tile map:", 50, "■");
                    idx.0 += 1;
                }
            })
        });
        TileMat {
            map,
            with_info: self.with_info,
            seed: self.seed,
            size: self.size,
        }
    }

    fn river_builder(
        &self,
        map: &mut Vec<Vec<Tile>>,
        x: usize,
        y: usize,
        direction: RiverDirection,
    ) {
        // Check the bound
        if ((x + 1) >= self.size
            || (y + 1) >= self.size
            || (x as i32 - 1) < 0
            || (y as i32 - 1) < 0)
        {
            return;
        } else if (map[x][y].tile_type == ShallowWater) {
            return;
        }

        // Set the new tile
        println!("Called here... x:{}, y:{}", x, y);
        map[x][y].tile_type = ShallowWater;

        let (mut north_side, mut south_side, mut east_side, mut west_side) = (2.0, 2.0, 2.0, 2.0);

        match direction {
            RiverDirection::None => {
                north_side = self.normalizer(self.map[x][y - 1]);
                south_side = self.normalizer(self.map[x][y + 1]);
                east_side = self.normalizer(self.map[x - 1][y]);
                west_side = self.normalizer(self.map[x + 1][y]);
            }
            RiverDirection::North => {
                south_side = self.normalizer(self.map[x][y + 1]);
                east_side = self.normalizer(self.map[x - 1][y]);
                west_side = self.normalizer(self.map[x + 1][y]);
            }
            RiverDirection::South => {
                north_side = self.normalizer(self.map[x][y - 1]);
                east_side = self.normalizer(self.map[x - 1][y]);
                west_side = self.normalizer(self.map[x + 1][y]);
            }
            RiverDirection::East => {
                north_side = self.normalizer(self.map[x][y - 1]);
                south_side = self.normalizer(self.map[x][y + 1]);
                east_side = self.normalizer(self.map[x - 1][y]);
            }
            RiverDirection::West => {
                north_side = self.normalizer(self.map[x][y - 1]);
                south_side = self.normalizer(self.map[x][y + 1]);
                west_side = self.normalizer(self.map[x + 1][y]);
            }
        }

        // Print the for direction value
        println!(
            "north:{:.2} south:{:.2} east:{:.2} west:{:.2}",
            north_side, south_side, east_side, west_side
        );

        // find the lowest point between these 4
        let mut mini = north_side.min(south_side);
        mini = mini.min(east_side);
        mini = mini.min(west_side);

        // Now that we have the minimum... let's check if it is actually the minimum
        println!("min: {:.2}", mini);

        // Now that we have the minimum, we need to setup the direction to go
        // Also need to check the bound...
        if (mini == north_side) {
            // We need to go north...
            // Recursive call to function with new coordinates
            self.river_builder(map, x, y - 1, RiverDirection::South);
        } else if (mini == south_side) {
            // We need to go south...
            self.river_builder(map, x, y + 1, RiverDirection::North);
        } else if (mini == east_side) {
            // We need to go east...
            self.river_builder(map, x - 1, y, RiverDirection::East);
        } else if (mini == west_side) {
            // We need to go west...
            self.river_builder(map, x + 1, y, RiverDirection::West);
        } else {
            // Like... we got nowhere to go...
            // Now we can place the water tile in the tile map
            return;
        }
    }

    fn normalizer(&self, num: f64) -> f64 {
        if num > 0.0 {
            num / self.max
        } else {
            -num / self.min
        }
    }
}

enum RiverDirection {
    None,
    North,
    South,
    East,
    West,
}

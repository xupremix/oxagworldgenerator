use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content::Water;
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Mountain, Sand, ShallowWater, Snow,
};
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
    let mut map = vec![vec![(0.0, false); size]; size];

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
        row.iter_mut().enumerate().for_each(|(x, (cell, _))| {
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
        mut self,
        spawn_levels: &OxAgTileTypeOptions,
        multiplier: f64,
    ) -> TileMat {
        let mut rng = StdRng::seed_from_u64(self.seed);

        let mut map = vec![
            vec![
                Tile {
                    tile_type: Grass,
                    content: Content::None,
                    elevation: 0,
                };
                self.size
            ];
            self.size
        ];

        let mut idx = (0, self.size.pow(2));
        self.map.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, (value, _))| {
                *value = if *value > 0.0 {
                    *value / self.max
                } else {
                    -*value / self.min
                };
                if map[i][j].tile_type != ShallowWater {
                    match *value {
                        value
                            if spawn_levels.deep_water_level.contains(&value)
                                | spawn_levels.shallow_water_level.contains(&value) =>
                        {
                            let tile_type = if spawn_levels.deep_water_level.contains(&value) {
                                DeepWater
                            } else {
                                ShallowWater
                            };
                            let content = Water(
                                rng.gen_range(0.0..Water(0).properties().max() as f64) as usize,
                            );
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
                            map[i][j].tile_type = Mountain;
                        }
                        value if spawn_levels.snow_level.contains(&value) => {
                            map[i][j].tile_type = Snow;
                        }
                        _ => {
                            let new_value = value.clone();
                            let dist_to_dw = (new_value
                                - (spawn_levels.deep_water_level.end()
                                    + spawn_levels.deep_water_level.start())
                                    / 2.0)
                                .abs();
                            let dist_to_sw = (new_value
                                - (spawn_levels.shallow_water_level.end()
                                    + spawn_levels.shallow_water_level.start())
                                    / 2.0)
                                .abs();
                            let dist_to_sd = (new_value
                                - (spawn_levels.sand_level.end()
                                    + spawn_levels.sand_level.start())
                                    / 2.0)
                                .abs();
                            let dist_to_gr = (new_value
                                - (spawn_levels.grass_level.end()
                                    + spawn_levels.grass_level.start())
                                    / 2.0)
                                .abs();
                            let dist_to_hl = (new_value
                                - (spawn_levels.hill_level.end()
                                    + spawn_levels.hill_level.start())
                                    / 2.0)
                                .abs();
                            let dist_to_mt = (new_value
                                - (spawn_levels.mountain_level.end()
                                    + spawn_levels.mountain_level.start())
                                    / 2.0)
                                .abs();

                            let dist_to_sn = (new_value
                                - (spawn_levels.snow_level.end()
                                    + spawn_levels.snow_level.start())
                                    / 2.0)
                                .abs();
                            let min = dist_to_dw
                                .min(dist_to_sw)
                                .min(dist_to_sd)
                                .min(dist_to_gr)
                                .min(dist_to_hl)
                                .min(dist_to_mt)
                                .min(dist_to_sn);
                            if min == dist_to_dw {
                                map[i][j].tile_type = DeepWater;
                            } else if min == dist_to_sw {
                                map[i][j].tile_type = ShallowWater;
                            } else if min == dist_to_sd {
                                map[i][j].tile_type = Sand;
                            } else if min == dist_to_gr {
                                map[i][j].tile_type = Grass;
                            } else if min == dist_to_hl {
                                map[i][j].tile_type = Hill;
                            } else if min == dist_to_mt {
                                map[i][j].tile_type = Mountain;
                            } else if min == dist_to_sn {
                                map[i][j].tile_type = Snow;
                            } else {
                                map[i][j].tile_type = Grass;
                            }
                        }
                    }
                }
                if self.with_info {
                    progress_bar(idx.0, idx.1, "Generating tile map:", 50, "■");
                    idx.0 += 1;
                }
            })
        });

        self.lava_spawn(&mut map, spawn_levels, &mut rng);

        for _ in 0..rng.gen_range(spawn_levels.river_n.clone()) {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while ![Hill, Mountain].contains(&map[row][col].tile_type) || self.map[row][col].1 {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }
            self.river_spawn(&mut map, row, col);
        }
        for _ in 0..rng.gen_range(spawn_levels.street_n.clone()) {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while ![Hill, Grass].contains(&map[row][col].tile_type) || self.map[row][col].1 {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }
            let n = rng.gen_range(spawn_levels.street_len.clone());
            let dir = match (rng.gen_range(0..4)) {
                0 => (0, 1),
                1 => (1, 0),
                2 => (0, -1),
                _ => (-1, 0),
            };
            self.street_spawn(&mut map, row, col, &mut rng, dir, n);
        }
        TileMat {
            map,
            with_info: self.with_info,
            seed: self.seed,
            size: self.size,
        }
    }
}

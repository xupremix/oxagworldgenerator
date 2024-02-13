use crate::utils::constants::{
    DEFAULT_NOISE_FREQUENCY, DEFAULT_NOISE_LACUNARITY, DEFAULT_NOISE_OCTAVES,
};
use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::circle_spawn::spawn_circle;
use crate::world_generator::spawning_tools::{MazeBuilder, TileMat};
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use noise::{Fbm, MultiFractal, NoiseFn, Perlin};
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::{Content, Tile, TileType};

pub(crate) fn maze_builder_init(seed: u64, size: usize) -> MazeBuilder {
    let perlin = Fbm::<Perlin>::new(seed as u32)
        .set_octaves(DEFAULT_NOISE_OCTAVES)
        .set_frequency(DEFAULT_NOISE_FREQUENCY)
        .set_lacunarity(DEFAULT_NOISE_LACUNARITY);

    MazeBuilder {
        seed,
        size,
        map: vec![
            vec![
                Tile {
                    tile_type: TileType::Wall,
                    content: Content::None,
                    elevation: 0,
                };
                size
            ];
            size
        ],
        perlin,
    }
}

pub(crate) fn random_tile(rng: &mut StdRng) -> Vec<TileType> {
    let mut tile_type = vec![
        TileType::Grass,
        TileType::ShallowWater,
        TileType::Sand,
        TileType::Snow,
    ];
    tile_type.shuffle(rng);
    tile_type
}

impl MazeBuilder {
    // maze builder
    pub(crate) fn builder(
        mut self,
        content_option: &Vec<(Content, OxAgContentOptions)>,
        tiletype_options: &OxAgTileTypeOptions,
    ) -> (Vec<Vec<Tile>>, (usize, usize)) {
        let rng = &mut StdRng::seed_from_u64(self.seed);
        let (spawn_x, spawn_y) = self.random_point(rng);
        self.maze_builder_loop(spawn_x as i32, spawn_y as i32, rng, tiletype_options);

        self.teleport_spawner(rng);
        self.spawn_end(rng);

        let tile_map = TileMat {
            map: self.map,
            with_info: false,
            seed: self.seed,
            size: self.size,
        };

        let result = tile_map.spawn_contents(content_option);
        (result.0.map, result.1)
    }

    fn teleport_spawner(&mut self, rng: &mut StdRng) {
        let max = rng.gen_range(0.0..(self.size as f32 * 0.1));
        if max < 1.0 {
            return;
        }
        for _ in 0..max as usize {
            let (x, y) = self.random_not_street(rng);
            self.map[y][x].content = Content::JollyBlock(1);
        }
    }

    fn random_not_street(&self, rng: &mut StdRng) -> (usize, usize) {
        let (mut x, mut y) = (
            rng.gen_range(1..self.size - 1),
            rng.gen_range(1..self.size - 1),
        );
        while self.map[y][x].tile_type == TileType::Wall {
            (x, y) = (
                rng.gen_range(1..self.size - 1),
                rng.gen_range(1..self.size - 1),
            );
        }
        (x, y)
    }

    // Path setter
    fn set_path(
        &mut self,
        x: usize,
        y: usize,
        spawn_levels: &OxAgTileTypeOptions,
        rng: &mut StdRng,
    ) {
        self.map[y][x].tile_type = TileType::Street;
        let (nx, ny) = (x as f64 / self.size as f64, y as f64 / self.size as f64);
        let value = self.perlin.get([nx, ny]);
        match value {
            value
                if spawn_levels.deep_water_level.contains(&value)
                    | spawn_levels.shallow_water_level.contains(&value) =>
            {
                let tile_type = if spawn_levels.deep_water_level.contains(&value) {
                    TileType::DeepWater
                } else {
                    TileType::ShallowWater
                };
                let content = Content::Water(
                    rng.gen_range(0.0..Content::Water(0).properties().max() as f64) as usize,
                );
                self.map[y][x] = Tile {
                    tile_type,
                    content,
                    elevation: 0,
                }
            }
            value if spawn_levels.sand_level.contains(&value) => {
                self.map[y][x].tile_type = TileType::Sand;
            }
            value if spawn_levels.hill_level.contains(&value) => {
                self.map[y][x].tile_type = TileType::Hill;
            }
            value if spawn_levels.mountain_level.contains(&value) => {
                self.map[y][x].tile_type = TileType::Mountain;
            }
            value if spawn_levels.snow_level.contains(&value) => {
                self.map[y][x].tile_type = TileType::Snow;
            }
            _ => {
                let new_value = value;
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
                    - (spawn_levels.sand_level.end() + spawn_levels.sand_level.start()) / 2.0)
                    .abs();
                let dist_to_gr = (new_value
                    - (spawn_levels.grass_level.end() + spawn_levels.grass_level.start()) / 2.0)
                    .abs();
                let dist_to_hl = (new_value
                    - (spawn_levels.hill_level.end() + spawn_levels.hill_level.start()) / 2.0)
                    .abs();
                let dist_to_mt = (new_value
                    - (spawn_levels.mountain_level.end() + spawn_levels.mountain_level.start())
                        / 2.0)
                    .abs();

                let dist_to_sn = (new_value
                    - (spawn_levels.snow_level.end() + spawn_levels.snow_level.start()) / 2.0)
                    .abs();
                let min = dist_to_dw
                    .min(dist_to_sw)
                    .min(dist_to_sd)
                    .min(dist_to_gr)
                    .min(dist_to_hl)
                    .min(dist_to_mt)
                    .min(dist_to_sn);
                if min == dist_to_dw {
                    self.map[y][x].tile_type = TileType::DeepWater;
                } else if min == dist_to_sw {
                    self.map[y][x].tile_type = TileType::ShallowWater;
                } else if min == dist_to_sd {
                    self.map[y][x].tile_type = TileType::Sand;
                } else if min == dist_to_gr {
                    self.map[y][x].tile_type = TileType::Grass;
                } else if min == dist_to_hl {
                    self.map[y][x].tile_type = TileType::Hill;
                } else if min == dist_to_mt {
                    self.map[y][x].tile_type = TileType::Mountain;
                } else if min == dist_to_sn {
                    self.map[y][x].tile_type = TileType::Snow;
                } else {
                    self.map[y][x].tile_type = TileType::Grass;
                }
            }
        }
    }
    // Check if where i want to place a path is wall
    fn is_wall(&self, x: usize, y: usize) -> TileType {
        if 0 < x && x < self.size && 0 < y && y < self.size {
            self.map[y][x].tile_type
        } else {
            TileType::Street
        }
    }
    // Random starting point chooser, based on the seed
    fn random_point(&self, rng: &mut StdRng) -> (usize, usize) {
        let (x, y) = (
            rng.gen_range(1..self.size - 1),
            rng.gen_range(1..self.size - 1),
        );
        (self.check_odd(x), self.check_odd(y))
    }
    //Spawn point need to be in an odd position
    fn check_odd(&self, mut num: usize) -> usize {
        if num % 2 == 0 {
            if num + 1 < self.size {
                num += 1;
            } else if num - 1 > 0 {
                num -= 1
            }
        }
        num
    }

    fn spawn_end(&mut self, rng: &mut StdRng) {
        let (x, y) = self.random_not_street(rng);
        self.map[y][x].content = Content::JollyBlock(1);
    }

    fn random_circles(&mut self, rng: &mut StdRng, tile: TileType) {
        let (spawn_x, spawn_y) = self.random_point(rng);
        let size = self.size as f32;
        let radius = rng.gen_range(1..size.sqrt() as usize);
        spawn_circle(
            &mut self.map,
            rng,
            self.size,
            spawn_x,
            spawn_y,
            radius,
            &(None, Some(tile)),
        );
        self.map[spawn_x][spawn_y].tile_type = TileType::Teleport(false);
    }

    fn maze_builder_loop(
        &mut self,
        start_x: i32,
        start_y: i32,
        mut rng: &mut StdRng,
        tiletype_options: &OxAgTileTypeOptions,
    ) {
        let mut stack: Vec<(i32, i32)> = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            // Set current cell to path
            self.set_path(x as usize, y as usize, tiletype_options, rng);

            // Create a list of direction that we can try and shuffle it
            let mut direction: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            direction.shuffle(&mut rng);

            while let Some(direction_to_try) = direction.pop() {
                // We calculate the new node's coordinates using the random direction
                let node_x = x + (direction_to_try.0 * 2);
                let node_y = y + (direction_to_try.1 * 2);

                // We check if the test node hasn't been visited
                if self.is_wall(node_x as usize, node_y as usize) == TileType::Wall {
                    // if it is successful: we found a path

                    // we set our linking call
                    let link_cell_x = x + direction_to_try.0;
                    let link_cell_y = y + direction_to_try.1;
                    self.set_path(
                        link_cell_x as usize,
                        link_cell_y as usize,
                        tiletype_options,
                        rng,
                    );

                    // Add the new coordinates to the stack instead of recursive call
                    stack.push((x, y));
                    stack.push((node_x, node_y));
                    break;
                }
            }
        }
    }
}

use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::circle_spawn::spawn_circle;
use crate::world_generator::spawning_tools::{MazeBuilder, TileMat};
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::{Content, Tile, TileType};

pub(crate) fn maze_builder_init(seed: u64, size: usize) -> MazeBuilder {
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
    ) -> (Vec<Vec<Tile>>, (usize, usize)) {
        let rng = &mut StdRng::seed_from_u64(self.seed);
        let (spawn_x, spawn_y) = self.random_point(rng);
        self.maze_builder_loop(spawn_x as i32, spawn_y as i32, rng);

        let mut tile_type = random_tile(rng);
        for _ in 0..(self.size as f32 * 0.3) as usize {
            if tile_type.len() <= 0 {
                tile_type = random_tile(rng);
            }
            let tile = tile_type.pop().unwrap();
            self.random_circles(rng, tile);
        }
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

    fn spawn_content(&mut self, rng: &mut StdRng) {}

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
        while self.map[y][x].tile_type != TileType::Street {
            (x, y) = (
                rng.gen_range(1..self.size - 1),
                rng.gen_range(1..self.size - 1),
            );
        }
        (x, y)
    }

    // Path setter
    fn set_path(&mut self, x: usize, y: usize) {
        self.map[y][x].tile_type = TileType::Street;
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

    fn maze_builder_loop(&mut self, start_x: i32, start_y: i32, mut rng: &mut StdRng) {
        let mut stack: Vec<(i32, i32)> = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            // Set current cell to path
            self.set_path(x as usize, y as usize);

            // Create a list of direction that we can try and shuffle it
            let mut direction: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            direction.shuffle(&mut rng);

            while let Some(direction_to_try) = direction.pop() {
                // We calculate the new node's coordinates using the random direction
                let node_x = x + (direction_to_try.0 * 2);
                let node_y = y + (direction_to_try.1 * 2);

                // We check if the test node hasn't been visited
                if self.is_wall(node_x as usize, node_y as usize) != TileType::Street {
                    // if it is successful: we found a path

                    // we set our linking call
                    let link_cell_x = x + direction_to_try.0;
                    let link_cell_y = y + direction_to_try.1;
                    self.set_path(link_cell_x as usize, link_cell_y as usize);

                    // Add the new coordinates to the stack instead of recursive call
                    stack.push((x, y));
                    stack.push((node_x, node_y));
                    break;
                }
            }
        }
    }
}

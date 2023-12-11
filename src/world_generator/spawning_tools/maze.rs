use crate::world_generator::spawning_tools::MazeBuilder;
use rand::prelude::StdRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::tile::TileType::Grass;
use serde::de::Unexpected::Option;
use crate::world_generator::spawning_tools::circle_spawn::spawn_circle;

pub(crate) fn maze_builder_init(seed: u64, mut size: usize) -> MazeBuilder {
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

impl MazeBuilder {
    // maze builder
    pub(crate) fn builder(mut self) -> (Vec<Vec<Tile>>, (usize, usize)) {
        let mut rng = &mut StdRng::seed_from_u64(self.seed);
        let (mut spawn_x, mut spawn_y) = self.starting_node(rng);
        self.maze_builder_loop(spawn_x as i32, spawn_y as i32, rng);
        let n_circle = self.size as f32;

        for _ in (0..(self.size as f32 * 0.3) as usize) {
            self.random_circles(rng);
        }

        self.spawn_end(rng);
        (self.map, (spawn_x, spawn_y))
    }
    // Path setter
    fn set_path(&mut self, x: usize, y: usize) {
        self.map[y][x].tile_type = TileType::Street;
    }
    // Check if where i want to place a path is wall
    fn is_wall(&self, x: usize, y: usize) -> TileType {
        if (0 < x && x < self.size && 0 < y && y < self.size) {
            self.map[y][x].tile_type
        } else {
            TileType::Street
        }
    }
    // Random starting point chooser, based on the seed
    fn starting_node(&self, rng: &mut StdRng) -> (usize, usize) {
        let (mut x, mut y) = (0, 0);
        (x, y) = (
            rng.gen_range(1..self.size - 1),
            rng.gen_range(1..self.size - 1),
        );
        (self.check_odd(x), self.check_odd(y))
    }
    //Spawn point need to be in an odd position
    fn check_odd(&self, mut num: usize) -> usize {
        if num % 2 == 0 {
            if (num + 1 < self.size) {
                num += 1;
            } else if (num - 1 > 0) {
                num -= 1
            }
        }
        num
    }

    fn already_set_path(&self, x: usize, y: usize) -> bool {
        if self.map[y][x].tile_type == TileType::Street {
            true
        } else {
            false
        }
    }

    fn maze_builder_loop(&mut self, start_x: i32, start_y: i32, mut rng: &mut StdRng) {
        let mut stack: Vec<(i32, i32)> = vec![(start_x, start_y)];

        while let Some((x, y)) = stack.pop() {
            // Set current cell to path
            self.set_path(x as usize, y as usize);

            // Create a list of direction that we can try and shuffle it
            let mut direction: Vec<(i32, i32)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
            direction.shuffle(&mut rng);

            while direction.len() > 0 {
                let direction_to_try = direction.pop().unwrap();
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

    fn spawn_end(&mut self, rng: &mut StdRng) {
        let (mut x, mut y) = (rng.gen_range(1..self.size -1), rng.gen_range(1..self.size-1));
        while self.map[y][x].tile_type != TileType::Street {
            (x, y) = (rng.gen_range(1..self.size -1), rng.gen_range(1..self.size-1));
        }
        self.map[y][x].content = Content::JollyBlock(1);
    }

    fn random_circles(&mut self, rng: &mut StdRng) {
        let (spawn_x, spawn_y) = self.starting_node(rng);
        let size = self.size as f32;
        let radius = rng.gen_range(1..size.sqrt() as usize);
        spawn_circle(&mut self.map, rng, self.size, spawn_x, spawn_y,radius, &(None, Some(Grass)));
        self.map[spawn_y][spawn_x].tile_type = TileType::Teleport(false);
    }
}

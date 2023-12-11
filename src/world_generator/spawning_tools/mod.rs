use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::HashMap;

use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::world_generator::get_tiletype_percentage;
use strum::IntoEnumIterator;

use crate::world_generator::content_options::OxAgContentOptions;

pub(crate) mod batch_spawn;
mod circle_spawn;
mod lava_spawn;
pub(crate) mod matrix_spawn;
pub(crate) mod maze;
pub(crate) mod random_spawn;
mod river_spawn;
mod street_spawn;

pub(crate) struct F64MatData {
    map: Vec<Vec<(f64, bool)>>,
    min: f64,
    max: f64,
    seed: u64,
    size: usize,
    with_info: bool,
}

pub(crate) struct MazeBuilder {
    seed: u64,
    size: usize,
    map: Vec<Vec<Tile>>,
}

pub(crate) struct TileMat {
    pub map: Vec<Vec<Tile>>,
    with_info: bool,
    seed: u64,
    size: usize,
}

impl TileMat {
    pub(crate) fn spawn_contents(
        mut self,
        content_options: &Vec<(Content, OxAgContentOptions)>,
    ) -> (Self, (usize, usize)) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let percentage_map = get_tiletype_percentage(&self.map);
        if self.with_info {
            println!("Spawning contents:")
        }
        for (content, content_option) in content_options.iter() {
            let content = &content.to_default();
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
                .sum::<f64>()
                * content_option.percentage;
            if content_option.is_present && percentage > 0.0 {
                if content_option.in_batches {
                    self.spawn_batches(content, content_option, percentage, &mut rng);
                } else {
                    self.spawn_randomly(content, content_option, percentage, &mut rng);
                }
            } else if self.with_info {
                println!("Skipping {:?}", content);
            }
        }
        self.choose_spawn(&mut rng)
    }

    fn choose_spawn(self, rng: &mut StdRng) -> (Self, (usize, usize)) {
        let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
        while !self.map[row][col].tile_type.properties().walk() {
            (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
        }
        (self, (row, col))
    }
}

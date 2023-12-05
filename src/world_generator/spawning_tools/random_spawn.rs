use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content;
use std::cmp::max;

use crate::utils::progress_bar;
use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::TileMat;

impl TileMat {
    pub(crate) fn spawn_randomly(
        &mut self,
        content: &Content,
        content_option: &OxAgContentOptions,
        percentage: f64,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let max_spawn_number = if content_option.with_max_spawn_number {
            content_option.max_spawn_number
        } else {
            let max = max(
                content_option.min_spawn_number,
                (self.size.pow(2) as f64 * percentage) as usize,
            );
            rng.gen_range(content_option.min_spawn_number..=max)
        };
        for i in 0..max_spawn_number {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while !self.map[row][col].tile_type.properties().can_hold(content) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size))
            }
            let mut value = 0;
            if content.properties().max() != 0 {
                value = rng.gen_range(0..content.properties().max());
            }
            self.map[row][col].content = content.to_value(value);
            if self.with_info {
                progress_bar(
                    i,
                    max_spawn_number,
                    &format!("Spawning {:?}:", content),
                    50,
                    "■",
                );
            }
        }
    }
}

use std::cmp::max;

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content;

use crate::utils::constants::DEFAULT_BATCH_DISTANCE;
use crate::utils::progress_bar;
use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::circle_spawn::spawn_circle;
use crate::world_generator::spawning_tools::TileMat;

impl TileMat {
    pub(crate) fn spawn_batches(
        &mut self,
        content: &Content,
        content_option: &OxAgContentOptions,
        percentage: f64,
        rng: &mut StdRng,
    ) {
        let max_rad = max(1, content_option.max_radius) as f64;
        let mut radius = max_rad;
        let max_spawn_number = if content_option.with_max_spawn_number {
            content_option.max_spawn_number
        } else {
            let max = max(
                content_option.min_spawn_number,
                ((self.size.pow(2) as f64 * percentage)
                    / (max_rad.powi(2) * 3.14 + DEFAULT_BATCH_DISTANCE as f64))
                    as usize,
            );
            rng.gen_range(content_option.min_spawn_number..=max)
        };
        for i in 0..max_spawn_number {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while !self.map[row][col].tile_type.properties().can_hold(content) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }
            radius = rng.gen_range(1.0..=max_rad);
            spawn_circle(
                &mut self.map,
                rng,
                self.size,
                row,
                col,
                radius as usize,
                &(Some(content.to_default()), None),
            );
            if self.with_info {
                progress_bar(
                    i,
                    max_spawn_number,
                    &format!("Spawning {:?} in batches:", content),
                    50,
                    "■",
                );
            }
        }
    }
}

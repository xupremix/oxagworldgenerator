use std::cmp::max;

use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content;

use crate::utils::constants::DEFAULT_BATCH_DISTANCE;
use crate::utils::progress_bar;
use crate::world_generator::content_options::OxAgContentOptions;
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
        let mut tot = 0.0;
        for i in 0..max_spawn_number {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while !self.map[row][col].tile_type.properties().can_hold(content) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }
            tot += radius * radius * std::f64::consts::PI;
            radius = rng.gen_range(1.0..=max_rad);
            self.spawn_circle(row, col, radius as usize, content);
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

    fn spawn_circle(&mut self, center_x: usize, center_y: usize, radius: usize, content: &Content) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let min_radius = radius.min(
            center_x
                .min(center_y)
                .min(self.size - center_x - 1)
                .min(self.size - center_y - 1),
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
            self.add(center_x + x, center_y + y, content);
            self.add(center_x + y, center_y + x, content);
            self.add(center_x - y, center_y + x, content);
            self.add(center_x - x, center_y + y, content);
            self.add(center_x - x, center_y - y, content);
            self.add(center_x - y, center_y - x, content);
            self.add(center_x + y, center_y - x, content);
            self.add(center_x + x, center_y - y, content);

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
                if (i - center_x).pow(2) + (j - center_y).pow(2) <= min_radius.pow(2) {
                    self.add(i, j, content);
                }
            }
        }
    }

    fn add(&mut self, row: isize, col: isize, content: &Content) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let mut value = 0;
        if content.properties().max() != 0 {
            value = rng.gen_range(0..content.properties().max());
        }
        let row = row as usize;
        let col = col as usize;
        if self.map[row][col].tile_type.properties().can_hold(content) {
            self.map[row][col].content = content.to_value(value);
        }
    }
}

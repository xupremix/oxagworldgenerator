use rand::prelude::StdRng;
use rand::Rng;
use rand::RngCore;
use robotics_lib::world::tile::Content;
use std::cmp::max;

use crate::utils::constants::DEFAULT_BATCH_DISTANCE;
use crate::world_generator::content_options::OxAgContentOptions;
use crate::world_generator::spawning_tools::matrix_spawn::f64_mat;
use crate::world_generator::spawning_tools::TileMat;

impl TileMat {
    // pub(crate) fn spawn_batches(
    //     &mut self,
    //     content: &Content,
    //     content_option: &OxAgContentOptions,
    //     percentage: f64,
    //     rng: &mut StdRng,
    // ) {
    //     let max_rad = max(1, content_option.max_radius) as f64;
    //     let mut radius = max_rad;
    //     let max_spawn_number = if content_option.with_max_spawn_number {
    //         content_option.max_spawn_number
    //     } else {
    //         let max = max(
    //             content_option.min_spawn_number,
    //             ((self.size.pow(2) as f64 * percentage)
    //                 / (max_rad.powi(2) * 3.14 + DEFAULT_BATCH_DISTANCE as f64))
    //                 as usize,
    //         );
    //         rng.gen_range(content_option.min_spawn_number..=max)
    //     };
    //     for i in 0..max_spawn_number {
    //         let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
    //         while !self.map[row][col].tile_type.properties().can_hold(content) {
    //             (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
    //         }
    //         radius = rng.gen_range(1.0..=max_rad);
    //         spawn_circle(
    //             &mut self.map,
    //             rng,
    //             self.size,
    //             row,
    //             col,
    //             radius as usize,
    //             &(Some(content.to_default()), None),
    //         );
    //         if self.with_info {
    //             progress_bar(
    //                 i,
    //                 max_spawn_number,
    //                 &format!("Spawning {:?} in batches:", content),
    //                 50,
    //                 "■",
    //             );
    //         }
    //     }
    // }

    pub(crate) fn spawn_batches(
        &mut self,
        content: &Content,
        content_option: &OxAgContentOptions,
        percentage: f64,
        rng: &mut StdRng,
    ) {
        // Get a random radius between 1 and the max
        let max_rad = max(1, content_option.max_radius) as f64;
        let radius = max_rad;
        // Set a maximum spawn number for the batches
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

        // Loop for spawn
        for _ in 0..max_spawn_number {
            // Get diameter random between (0 and radius) * 2
            let diameter = (radius * 2.0) as usize;
            // Get the center
            let center = diameter / 2;
            // Create a matrix of size diameter
            let batches_noise = f64_mat(self.seed + rng.next_u32() as u64, diameter, false);

            // i check to find a tile that can hold the content on the map, the center of the batch spawn
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while !self.map[row][col].tile_type.properties().can_hold(content) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }

            // Start to iter the batch noise map
            batches_noise
                .map
                .iter()
                .enumerate()
                .for_each(|(tmp_row, rows)| {
                    rows.iter().enumerate().for_each(|(tmp_col, cell)| {
                        let is_circle = (tmp_row as isize - center as isize).pow(2)
                            + (tmp_col as isize - center as isize).pow(2)
                            <= radius.powf(2.0) as isize;

                        if is_circle {
                            if (row as f64 + tmp_row as f64 - radius) >= 0.0
                                && (col as f64 + tmp_col as f64 - radius) >= 0.0
                                && (row as f64 + tmp_row as f64 - radius) < self.size as f64
                                && (col as f64 + tmp_col as f64 - radius) < self.size as f64
                            {
                                let (row_map, col_map) = (
                                    row + tmp_row - radius as usize,
                                    col + tmp_col - radius as usize,
                                );

                                let percent_noise = (cell.0 - batches_noise.min) / (batches_noise.max - batches_noise.min);
                                let distance = ((tmp_row as f64 - center as f64).powf(2.0) + (tmp_col as f64 - center as f64).powf(2.0)).sqrt();
                                let percent_distance = (distance / center as f64);
                                let probability = (percent_distance + percent_noise) / 2.0;

                                if rng.gen_bool(probability) {
                                    let mut value = 0;
                                    if content.properties().max() != 0 {
                                        value = rng.gen_range(0..content.properties().max());

                                        if self.map[row_map][col_map]
                                            .tile_type
                                            .properties()
                                            .can_hold(content)
                                        {
                                            self.map[row_map][col_map].content = content.to_value(value)
                                        }
                                    }
                                }
                            }
                        }
                    })
                })
        }
    }
}

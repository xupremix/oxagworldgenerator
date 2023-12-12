use std::cmp::max;
use rand::prelude::StdRng;
use rand::RngCore;
use rand::{Rng};
use robotics_lib::world::tile::Content;

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
        let max_rad = max(1, content_option.max_radius) as f64;
        let radius = max_rad;
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

        for _ in 0..max_spawn_number {
            let batches_noise = f64_mat(
                self.seed + rng.next_u32() as u64,
                rng.gen_range(1..=(radius as usize)),
                false,
            );

            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while !self.map[row][col].tile_type.properties().can_hold(content) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }

            batches_noise
                .map
                .iter()
                .enumerate()
                .for_each(|(tmp_row, rows)| {
                    rows.iter().enumerate().for_each(|(tmp_col, cell)| {
                        if (-0.25..=0.25).contains(&cell.0) {
                            if !(((row as i32 + tmp_row as i32 - radius as i32) as usize) < 0
                                || ((col as i32 + tmp_col as i32 - radius as i32) as usize) < 0
                                || ((row as i32 + tmp_row as i32 - radius as i32) as usize)
                                    > self.size
                                || ((col as i32 + tmp_col as i32 - radius as i32) as usize)
                                    > self.size)
                            {
                                let (row, col) = (
                                    row + tmp_row - radius as usize,
                                    col + tmp_col - radius as usize,
                                );
                                if self.map[row][col].tile_type.properties().can_hold(content) {
                                    // println!("Placed Stuff: {}", placed_stuff);
                                    // placed_stuff +=1;
                                    let value: usize = if row > content.properties().max() {
                                        content.properties().max()
                                    } else {
                                        rng.gen_range(row..=content.properties().max())
                                    };
                                    self.map[row][col].content = content.to_value(value);
                                }
                            }
                        }
                    })
                })
        }
    }
}

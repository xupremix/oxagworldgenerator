use crate::utils::progress_bar;
use crate::world_generator::spawning_tools::TileMat;
use crate::world_generator::tile_content_spawn_options::OxAgTileContentSpawnOptions;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content;

impl TileMat {
    pub(crate) fn spawn_randomly(
        &mut self,
        content: &Content,
        content_option: &OxAgTileContentSpawnOptions,
        percentage: f64,
    ) {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let max_spawn_number = if content_option.with_max_spawn_number {
            content_option.max_spawn_number
        } else {
            rng.gen_range(
                content_option.min_spawn_number..(self.size.pow(2) as f64 * percentage) as usize,
            )
        };
        for i in 0..max_spawn_number {
            let mut row = 0;
            let mut col = 0;
            loop {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
                if self.map[row][col].tile_type.properties().can_hold(content) {
                    break;
                }
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
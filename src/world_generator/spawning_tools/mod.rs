use crate::world_generator::tile_content_spawn_options::OxAgTileContentSpawnOptions;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::get_tiletype_percentage;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub(crate) mod batch_spawn;
pub(crate) mod matrix_spawn;
pub(crate) mod random_spawn;

pub(crate) struct F64MatData {
    map: Vec<Vec<f64>>,
    min: f64,
    max: f64,
    seed: u64,
    size: usize,
    with_info: bool,
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
        content_options: &HashMap<Content, OxAgTileContentSpawnOptions>,
    ) -> Self {
        let percentage_map = get_tiletype_percentage(&self.map);

        // let contents = self
        //     .tile_content_spawn_options
        //     .iter()
        //     .collect::<Vec<(&Content, &OxAgTileContentSpawnOptions)>>()
        //     .sort();
        if self.with_info {
            println!("Spawning contents:")
        }
        for (content, content_option) in content_options.iter() {
            let content = &content.to_default();
            if content_option.is_present {
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
                    .sum::<f64>();
                if content_option.in_batches {
                    self.spawn_batches(content, content_option, percentage);
                } else {
                    self.spawn_randomly(content, content_option, percentage);
                }
            } else if self.with_info {
                println!("Skipping {:?}", content);
            }
        }
        self
    }
}

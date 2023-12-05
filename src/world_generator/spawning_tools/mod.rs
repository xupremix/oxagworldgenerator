use robotics_lib::world::tile::TileType::Grass;
use robotics_lib::world::tile::{Content, Tile, TileType};
use robotics_lib::world::worldgenerator::get_tiletype_percentage;
use std::collections::HashMap;
use strum::IntoEnumIterator;

use crate::world_generator::content_options::OxAgContentOptions;

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
        content_options: &Vec<(Content, OxAgContentOptions)>,
        content_spawn_percent: &HashMap<Content, f64>,
    ) -> Self {
        let percentage_map = get_tiletype_percentage(&self.map);
        if self.with_info {
            println!("Spawning contents:")
        }

        //println!("Grass percentage {}", percentage_map.get(&Grass).unwrap());
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
                * content_spawn_percent.get(content).unwrap_or(&1.0);
            if content_option.is_present && percentage > 0.0 {
                if content_option.in_batches {
                    println!(
                        "Content: {:?}, percentage of tiles: {}",
                        content, percentage
                    );
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

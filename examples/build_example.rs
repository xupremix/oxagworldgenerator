use rand::Rng;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Mountain, Sand, ShallowWater, Snow, Street, Wall,
};
use std::time::{Duration, Instant};

use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::world::world_generator::Generator;

fn main() {
    let size: usize = 256;
    let seed = 451; // generate_random_seed();
    let start = Instant::now();
    //
    // let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
    //     .set_size(size)
    //     .set_maze(true)
    //     .build()
    //     .unwrap();

    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::WaterWorld)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_with_info(true)
        .set_maze(false)
        .build()
        .unwrap();

    let tmp = generator.gen();
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
    /*
    tmp.0.iter().enumerate().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| {
            if cell.content.to_default() == Content::JollyBlock(0) {
                print!("EE");
            } else if (x == tmp.1 .0 && y == tmp.1 .1) {
                print!("SS");
            } else {
                match cell.tile_type {
                    DeepWater => print!("  "),
                    ShallowWater => print!(" "),
                    Sand => print!(" "),
                    Grass => print!("GG"),
                    Hill => print!("  "),
                    Mountain => print!("  "),
                    Snow => print!("  "),
                    Wall => print!("██"),
                    Street => print!("  "),
                    Teleport => print!("TT"),
                }
            }
        });

        println!();
    })*/
}

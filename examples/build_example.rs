use robotics_lib::world::tile::TileType::{
    DeepWater, Grass, Hill, Mountain, Sand, ShallowWater, Snow, Street, Wall,
};
use std::time::{Duration, Instant};

use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;
use robotics_lib::world::worldgenerator::Generator;

fn main() {
    let size: usize = 32;
    let seed = 451; // generate_random_seed();
    let start = Instant::now();

    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_maze(true)
        .build();

    let tmp = generator.gen().0;
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);

    tmp.iter().enumerate().for_each(|(y, row)| {
        row.iter()
            .enumerate()
            .for_each(|(x, cell)| match cell.tile_type {
                DeepWater => print!("  "),
                ShallowWater => print!("WW"),
                Sand => print!("KK"),
                Grass => print!("00"),
                Hill => print!("  "),
                Mountain => print!("  "),
                Snow => print!("  "),
                Wall => print!("██"),
                Street => print!("SS"),
                _ => print!("  "),
            });
        println!();
    })
}

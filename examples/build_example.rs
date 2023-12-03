use std::time::{Duration, Instant};

use robotics_lib::world::worldgenerator::Generator;

use lib_oxidizing_agents::world_generator::presets::content_presets::OxAgContentPresets;
use lib_oxidizing_agents::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use lib_oxidizing_agents::world_generator::OxAgWorldGenerator;

fn main() {
    let size: usize = 1024;
    let seed = 44445; // generate_random_seed();
    let start = Instant::now();
    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::WaterWorld)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_with_info(true)
        .build();

    let tmp = generator.gen().0;
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

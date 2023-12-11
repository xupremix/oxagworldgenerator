use std::time::{Duration, Instant};

use oxagworldgenerator::world_generator::presets::content_presets::OxAgContentPresets;
use oxagworldgenerator::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use oxagworldgenerator::world_generator::presets::tile_type_presets::OxAgTileTypePresets;
use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;

fn main() {
    let size: usize = 256;
    let seed = 751776;
    let start = Instant::now();
    let mut generator= OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .set_environmental_conditions_from_preset(OxAgEnvironmentalConditionPresets::Sunny)
        .set_tile_type_options_from_preset(OxAgTileTypePresets::Default)
        .set_content_options_from_preset(OxAgContentPresets::Default)
        .set_with_info(true)
        .build().unwrap();

    generator.save("examples/save.json").unwrap();

    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

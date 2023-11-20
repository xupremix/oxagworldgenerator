use lib_oxidizing_agents;
use lib_oxidizing_agents::worldgenerator::content_gen_options::OxAgContentGenerationPresets;
use lib_oxidizing_agents::worldgenerator::world_gen_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};
use lib_oxidizing_agents::worldgenerator::OxAgWorldGenerator;
use robotics_lib;

fn main() {
    let size = 256;
    let seed = 42;
    let generator = OxAgWorldGenerator::init()
        .set_seed(seed)
        .set_size(size)
        .gen_world_options_from_preset(OxAgWorldGenerationPresets::DEFAULT);

    let other_gen = OxAgWorldGenerator::new(seed)
        .set_size(size)
        .set_world_gen_options(OxAgWorldGenerationOptions {
            deep_water_level: -1.0..=-0.5,
            shallow_water_level: -0.5..=0.0,
            sand_level: 0.0..=0.2,
            grass_level: 0.2..=0.4,
            hill_level: 0.4..=0.6,
            mountain_level: 0.6..=0.8,
            snow_level: 0.8..=1.0,
        })
        .unwrap()
        .gen_content_options_from_preset(OxAgContentGenerationPresets::DEFAULT);

    // cool generation implementation using a functional approach
}

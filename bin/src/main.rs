use lib_oxidizing_agents;
use lib_oxidizing_agents::worldgenerator::content_gen_options::{
    OxAgContentGenerationPresets, OxAgContentOption,
};
use lib_oxidizing_agents::worldgenerator::world_gen_options::{
    OxAgWorldGenerationOptions, OxAgWorldGenerationPresets,
};
use lib_oxidizing_agents::worldgenerator::OxAgWorldGenerator;
use robotics_lib;
use robotics_lib::world::environmental_conditions::EnvironmentalConditions;
use robotics_lib::world::environmental_conditions::WeatherType::Sunny;
use robotics_lib::world::tile::Content;

fn main() {
    let size = 256;
    let seed = 42;
    let generator = OxAgWorldGenerator::new(seed)
        .set_size(size)
        .gen_world_options_from_preset(OxAgWorldGenerationPresets::DEFAULT)
        .alter_content_gen_options(
            Content::Tree(0),
            OxAgContentOption {
                in_batches: false,
                present: false,
                min_spawn_number: 0,
                spawn_level: 0.0,
            },
        )
        .unwrap();
}

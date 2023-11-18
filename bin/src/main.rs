use lib_oxidizing_agents;
use lib_oxidizing_agents::worldgenerator::world_gen_options::OxAgWorldGenerationOptions;
use lib_oxidizing_agents::worldgenerator::OxAgWorldGenerator;
use robotics_lib;

fn main() {
    let size = 256;
    let seed = 42;
    let generator = OxAgWorldGenerator::default()
        .set_size(size)
        .set_seed(seed)
        .set_world_gen_options(
            lib_oxidizing_agents::worldgenerator::world_gen_options::presets::DEFAULT,
        )
        .unwrap();
    // cool generation implementation using a functional approach
}

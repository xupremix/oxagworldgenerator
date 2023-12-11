use lib_oxidizing_agents::world_generator::presets::environmental_presets::OxAgEnvironmentalConditionPresets;
use lib_oxidizing_agents::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use robotics_lib::world::world_generator::Generator;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let mut generator = OxAgWorldGeneratorBuilder::new()
        .load("examples/save.json")
        .unwrap();
    let tmp = generator.gen().0;
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

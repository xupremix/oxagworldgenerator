use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use robotics_lib::world::world_generator::Generator;
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    let mut generator = OxAgWorldGeneratorBuilder::new()
        .load("examples/save.json")
        .unwrap();
    let _ = generator.gen().0;
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

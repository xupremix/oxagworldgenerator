use oxagworldgenerator::world_generator::world_generator_builder::OxAgWorldGeneratorBuilder;
use oxagworldgenerator::world_generator::OxAgWorldGenerator;
use robotics_lib::world::world_generator::Generator;
use std::time::{Duration, Instant};

fn main() {
    let size: usize = 1024;
    let seed = 420;
    let start = Instant::now();

    let mut generator: OxAgWorldGenerator = OxAgWorldGeneratorBuilder::new()
        .set_seed(seed)
        .set_size(size)
        .build()
        .unwrap();

    let tmp = generator.gen();
    let duration: Duration = start.elapsed();
    println!("Duration: {:?}", duration);
}

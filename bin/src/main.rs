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
use std::error::Error;
use std::fmt::format;
use std::fs::File;
use std::io::Write;
use std::process::Command;

fn main() {
    let size = 512;
    let seed = 46;
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

    let tmp = generator.call_build();

    let mut file = File::create("src/executables/data.py").unwrap();

    println!("Finished building the map, now writing to file ... ");

    file.write("table = [\n".as_bytes());
    tmp.iter().enumerate().for_each(|(i, row)| {
        file.write("[".as_bytes());
        row.iter().enumerate().for_each(|(j, elem)| {
            file.write(format!("{:.5}, ", elem).as_bytes());
        });
        file.write("], \n".as_bytes());
    });
    file.write("]\n".as_bytes());

    println!("Finished writing to file, now running the python script ... ");
    let python_script = "src/executables/displayer.py";
    let output = Command::new("python")
        .arg(python_script)
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        // Get the output as a string
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("Python script output:\n{}", output_str);
    } else {
        // Print any error message if the command failed
        let error_str = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error executing Python script:\n{}", error_str);
    }
}

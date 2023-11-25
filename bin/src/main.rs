use std::fs::File;
use std::io::Write;
use std::process::Command;

use robotics_lib;

use lib_oxidizing_agents;
use lib_oxidizing_agents::utils::{gen_seed, OxAgError};
use lib_oxidizing_agents::worldgenerator::OxAgWorldGeneratorBuilder;

fn main() {
    let size = 256;
    let seed = gen_seed();
    let generator = OxAgWorldGeneratorBuilder::new().set_size(size).build();

    let tmp = generator.gen_map();

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

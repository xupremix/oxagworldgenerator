use lib_oxidizing_agents;
use lib_oxidizing_agents::tools::OxTool;
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use noise::{Add, Fbm, MultiFractal, NoiseFn, Perlin, Seedable};
use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::runner::{run, Robot, Runnable};
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;

struct MyRobot {
    robot: Robot,
    ox_tool: OxTool,
}

impl Runnable for MyRobot {
    fn process_tick(&mut self, world: &mut World) {
        // logic
    }

    fn get_energy(&self) -> &Energy {
        &self.robot.energy
    }

    fn get_energy_mut(&mut self) -> &mut Energy {
        &mut self.robot.energy
    }

    fn get_coordinate(&self) -> &Coordinate {
        &self.robot.coordinate
    }

    fn get_coordinate_mut(&mut self) -> &mut Coordinate {
        &mut self.robot.coordinate
    }

    fn get_backpack(&self) -> &BackPack {
        &self.robot.backpack
    }

    fn get_backpack_mut(&mut self) -> &mut BackPack {
        &mut self.robot.backpack
    }
}

fn main() {
    // let mut robot = MyRobot {
    //     robot: Robot::new(),
    //     ox_tool: OxTool::new(),
    // };
    // generator
    let fbm = Fbm::<Perlin>::new(0).set_octaves(2);
    let fbm2 = Fbm::<Perlin>::new(0).set_octaves(10);

    let fmb_final = Add::<f64, Fbm<Perlin>, Fbm<Perlin>, 2>::new(fbm, fbm2);

    let noisemap = PlaneMapBuilder::new(fmb_final)
        .set_size(10, 10)
        .set_x_bounds(-1.0, 1.0)
        .set_y_bounds(-1.0, 1.0)
        .build();
    for i in 0..10 {
        for j in 0..10 {
            print!("\t{:.2}", noisemap.get_value(i, j));
        }
        println!();
    }
    // run(robot)
}

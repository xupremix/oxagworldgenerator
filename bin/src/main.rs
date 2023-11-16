use robotics_lib;
use robotics_lib::energy::Energy;
use robotics_lib::runner::{Robot, run, Runnable};
use robotics_lib::runner::backpack::BackPack;
use robotics_lib::world::coordinates::Coordinate;
use robotics_lib::world::World;
use lib_oxidizing_agents;
use lib_oxidizing_agents::tools::OxTool;

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
    let mut robot = MyRobot {
        robot: Robot::new(),
        ox_tool: OxTool::new(),
    };
    // generator

    // run(robot)
}

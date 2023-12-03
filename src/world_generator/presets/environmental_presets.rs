use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};

use crate::utils::traits::Loadable;

pub enum OxAgEnvironmentalConditionPresets {
    Sunny,
    Mixed,
}

impl Loadable<EnvironmentalConditions> for OxAgEnvironmentalConditionPresets {
    fn load(&self) -> EnvironmentalConditions {
        match self {
            OxAgEnvironmentalConditionPresets::Sunny => SUNNY(),
            OxAgEnvironmentalConditionPresets::Mixed => MIXED(),
        }
    }
}
const SUNNY: fn() -> EnvironmentalConditions =
    || EnvironmentalConditions::new(&[WeatherType::Sunny], 1, 0).unwrap();

const MIXED: fn() -> EnvironmentalConditions = || {
    EnvironmentalConditions::new(
        &[WeatherType::Sunny, WeatherType::Foggy, WeatherType::Rainy],
        1,
        0,
    )
    .unwrap()
};

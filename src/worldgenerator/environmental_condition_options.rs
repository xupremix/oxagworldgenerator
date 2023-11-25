use crate::utils::WEATHER_PROBABILITY;
use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use strum::IntoEnumIterator;

pub enum EnvironmentalConditionsPresets {
    DEFAULT,
}

pub struct OxAgEnvironmentalConditionsOptions;
impl OxAgEnvironmentalConditionsOptions {
    pub fn new(seed: u64) -> EnvironmentalConditions {
        let mut rng = StdRng::seed_from_u64(seed);
        let vec = WeatherType::iter()
            .filter(|_| rng.gen_bool(WEATHER_PROBABILITY))
            .collect::<Vec<WeatherType>>();
        EnvironmentalConditions::new(&vec, rng.gen::<u8>(), rng.gen::<u8>())
    }
}

impl EnvironmentalConditionsPresets {
    pub fn from_preset(preset: Self) -> EnvironmentalConditions {
        match preset {
            EnvironmentalConditionsPresets::DEFAULT => presets::DEFAULT(),
        }
    }
}

pub(crate) mod presets {
    use robotics_lib::world::environmental_conditions::EnvironmentalConditions;

    pub const DEFAULT: fn() -> EnvironmentalConditions = || EnvironmentalConditions::new(&[], 0, 0);
}

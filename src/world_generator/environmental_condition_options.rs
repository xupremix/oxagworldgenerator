use rand::prelude::StdRng;
use rand::Rng;
use rand::SeedableRng;
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use strum::IntoEnumIterator;

use super::constants::DEFAULT_WEATHER_PROBABILITY;

/// A wrapper around [EnvironmentalConditions] for ease of use.
#[derive(Debug, Clone)]
pub struct OxAgEnvironmentalConditions(EnvironmentalConditions);

impl OxAgEnvironmentalConditions {
    /// Utility constructor for the wrapper [EnvironmentalConditions].
    pub fn new_from_environmental_conditions(
        environmental_conditions: EnvironmentalConditions,
    ) -> OxAgEnvironmentalConditions {
        OxAgEnvironmentalConditions(environmental_conditions)
    }

    /// Calculates [OxAgEnvironmentalConditions] from a given `seed`
    pub fn new_from_seed(seed: u64) -> OxAgEnvironmentalConditions {
        let mut rng = StdRng::seed_from_u64(seed);

        let vec = WeatherType::iter()
            .filter(|_| rng.gen_bool(DEFAULT_WEATHER_PROBABILITY))
            .collect::<Vec<WeatherType>>();

        OxAgEnvironmentalConditions(EnvironmentalConditions::new(
            &vec,
            rng.gen::<u8>(),
            rng.gen::<u8>(),
        ))
    }

    /// Calculates [OxAgEnvironmentalConditions] from a given `wrapper`
    pub fn new_from_preset(
        preset: OxAgEnvironmentalConditionsPresets,
    ) -> OxAgEnvironmentalConditions {
        match preset {
            OxAgEnvironmentalConditionsPresets::DEFAULT => presets::DEFAULT(),
        }
    }
}

impl Into<EnvironmentalConditions> for OxAgEnvironmentalConditions {
    fn into(self) -> EnvironmentalConditions {
        self.0
    }
}

/// Presets of [OxAgEnvironmentalConditions] that can be used in the world generator.
pub enum OxAgEnvironmentalConditionsPresets {
    DEFAULT,
}

pub(crate) mod presets {
    use robotics_lib::world::environmental_conditions::EnvironmentalConditions;

    use super::OxAgEnvironmentalConditions;

    pub const DEFAULT: fn() -> OxAgEnvironmentalConditions =
        || OxAgEnvironmentalConditions(EnvironmentalConditions::new(&[], 0, 0));
}

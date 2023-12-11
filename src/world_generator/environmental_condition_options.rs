use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::environmental_conditions::{EnvironmentalConditions, WeatherType};
use serde::Serializer;
use strum::IntoEnumIterator;

use crate::utils::constants::DEFAULT_WEATHER_PROBABILITY;
use crate::utils::traits::FromSeed;

impl FromSeed for EnvironmentalConditions {
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        EnvironmentalConditions::new(
            &WeatherType::iter()
                .filter(|_| rng.gen_bool(DEFAULT_WEATHER_PROBABILITY))
                .collect::<Vec<WeatherType>>(),
            rng.gen::<u8>(),
            rng.gen_range(0..=24),
        )
        .unwrap()
    }
}

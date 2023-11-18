use std::ops::RangeInclusive;

pub struct OxAgWorldGenerationOptions {
    pub deep_water_level: RangeInclusive<f64>,
    pub shallow_water_level: RangeInclusive<f64>,
    pub sand_level: RangeInclusive<f64>,
    pub grass_level: RangeInclusive<f64>,
    pub hill_level: RangeInclusive<f64>,
    pub mountain_level: RangeInclusive<f64>,
    pub snow_level: RangeInclusive<f64>,
    // other implementation for street and lava tiles
}

impl OxAgWorldGenerationOptions {
    pub fn is_valid(&self) -> bool {
        self.deep_water_level.start() <= self.deep_water_level.end()
            && self.shallow_water_level.start() <= self.shallow_water_level.end()
            && self.sand_level.start() <= self.sand_level.end()
            && self.grass_level.start() <= self.grass_level.end()
            && self.hill_level.start() <= self.hill_level.end()
            && self.mountain_level.start() <= self.mountain_level.end()
            && self.snow_level.start() <= self.snow_level.end()
            && self.deep_water_level.end() <= self.shallow_water_level.start()
            && self.shallow_water_level.end() <= self.sand_level.start()
            && self.grass_level.end() <= self.hill_level.start()
            && self.hill_level.end() <= self.mountain_level.start()
            && self.mountain_level.end() <= self.snow_level.start()
    }
    pub fn validate(world_generation_options: &OxAgWorldGenerationOptions) -> bool {
        world_generation_options.is_valid()
    }
}

impl Default for OxAgWorldGenerationOptions {
    fn default() -> Self {
        Self {
            deep_water_level: -1.0..=0.0,
            shallow_water_level: 0.0..=0.0,
            sand_level: 0.0..=0.0,
            grass_level: 0.0..=0.0,
            hill_level: 0.0..=0.0,
            mountain_level: 0.0..=0.0,
            snow_level: 0.0..=1.0,
        }
    }
}

pub mod presets {
    use crate::worldgenerator::world_gen_options::OxAgWorldGenerationOptions;

    pub const DEFAULT: OxAgWorldGenerationOptions = OxAgWorldGenerationOptions {
        deep_water_level: -1.0..=-0.75,
        shallow_water_level: -0.75..=-0.5,
        sand_level: -0.5..=-0.25,
        grass_level: -0.25..=0.25,
        hill_level: 0.25..=0.5,
        mountain_level: 0.5..=0.75,
        snow_level: 0.75..=1.0,
    };
    pub const WATER_WORLD: OxAgWorldGenerationOptions = OxAgWorldGenerationOptions {
        deep_water_level: -1.0..=-0.5,
        shallow_water_level: -0.5..=0.0,
        sand_level: 0.0..=0.2,
        grass_level: 0.2..=0.4,
        hill_level: 0.4..=0.6,
        mountain_level: 0.6..=0.8,
        snow_level: 0.8..=1.0,
    };
}

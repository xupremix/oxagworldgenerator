use robotics_lib::world::tile::Content;

// TODO: Describe errors
#[derive(Debug, Clone)]
pub enum OxAgError {
    SizeNotSet,
    SeedNotSet,
    WorldOptionsNotSet,
    ContentOptionsNotSet,
    WeatherOptionsNotSet,
    InvalidWorldGenerationOption,
    InvalidContentGenerationOption(Content),
    ContentOptionNotSet(Content),
    CannotSetContentOptionForNone,
    WrongLowerBound,
    WrongUpperBound,
    InvalidSpawnLevel(Content),
    RangesAreOutOfBounds,
    InvalidContentSpawnOption(Content),
}

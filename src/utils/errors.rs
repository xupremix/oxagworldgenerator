use robotics_lib::world::tile::Content;

// TODO: Describe errors
#[derive(Debug, Clone)]
pub enum OxAgError {
    SizeNotSet,
    SeedNotSet,
    WorldOptionsNotSet,
    ContentOptionsNotSet,
    WrongLowerBound,
    WrongUpperBound,
    RangesAreOutOfBounds,
    InvalidContentOption(Content),
    InvalidContentOptionProvided,
}

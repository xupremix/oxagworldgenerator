use crate::utils::errors::OxAgError;
use crate::utils::errors::OxAgError::InvalidContentSpawnOption;
use crate::utils::traits::{Loadable, Validator};
use crate::world_generator::presets::content_presets::OxAgContentPresets;
use robotics_lib::world::tile::Content;
use robotics_lib::world::tile::Content::*;
use std::collections::HashMap;
use strum::IntoEnumIterator;

pub enum OxAgContentSpawnPresets {
    Default,
    None,
}

impl Loadable<HashMap<Content, f64>> for OxAgContentSpawnPresets {
    fn load(&self) -> HashMap<Content, f64> {
        match self {
            OxAgContentSpawnPresets::None => NONE(),
            OxAgContentSpawnPresets::Default => DEFAULT(),
        }
    }
}

const DEFAULT: fn() -> HashMap<Content, f64> = || {
    HashMap::from([
        (Rock(0), 0.07),
        (Tree(0), 0.15),
        (Garbage(0), 0.07),
        (Fire, 0.04),
        (Coin(0), 0.07),
        (Bin(0..0), 0.01),
        (Crate(0..0), 0.01),
        (Bank(0..0), 0.01),
        (Water(0), 0.0),
        (Market(0), 0.01),
        (Fish(0), 0.07),
        (Building, 0.01),
        (Bush(0), 0.07),
        (JollyBlock(0), 0.01),
        (Scarecrow, 0.01),
        (None, 0.0),
    ])
};

impl Validator for HashMap<Content, f64> {
    fn validate(&self) -> Result<(), OxAgError> {
        let mut out = Content::None;
        if self.iter().any(|(c, p)| {
            out = c.to_default();
            *p <= 0.0 || *p >= 1.0
        }) {
            Err(InvalidContentSpawnOption(out))
        } else {
            Ok(())
        }
    }
}

const NONE: fn() -> HashMap<Content, f64> =
    || Content::iter().map(|c| (c.to_default(), 0.0)).collect();

use std::ops::RangeInclusive;

use crate::utils::errors::OxAgError;

pub trait FromSeed {
    fn new_from_seed(seed: u64) -> Self;
}

pub trait Loadable<T> {
    fn load(&self) -> T;
}

/// trait to check if some data is within some other data
pub trait Container<C> {
    fn within(&self, range: &C) -> bool;
}

impl Container<RangeInclusive<f64>> for RangeInclusive<f64> {
    fn within(&self, range: &RangeInclusive<f64>) -> bool {
        (range.start() <= self.start()) && (range.end() >= self.end())
    }
}

pub trait Validator {
    fn validate(&self) -> Result<(), OxAgError>;
}

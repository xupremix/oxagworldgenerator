use std::ops::RangeInclusive;

pub mod errors;

/// trait to check if some data is within some other data
pub trait Container<C> {
    fn within(&self, range: &C) -> bool;
}

impl Container<RangeInclusive<f64>> for RangeInclusive<f64> {
    fn within(&self, range: &RangeInclusive<f64>) -> bool {
        (range.start() <= self.start()) && (range.end() >= self.end())
    }
}

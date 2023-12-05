use crate::utils::traits::FromSeed;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::Content;
use std::collections::HashMap;
use strum::IntoEnumIterator;

impl FromSeed for HashMap<Content, f64> {
    fn new_from_seed(seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);

        let mut content_map = HashMap::new();
        Content::iter().for_each(|c| {
            content_map.insert(c.to_default(), rng.gen_range(0.0..1.0));
        });

        content_map
    }
}

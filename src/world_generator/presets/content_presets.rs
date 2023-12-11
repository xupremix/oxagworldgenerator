use robotics_lib::world::tile::Content;
use strum::IntoEnumIterator;

use crate::utils::traits::Loadable;
use crate::world_generator::content_options::OxAgContentOptions;

pub enum OxAgContentPresets {
    None,
    Default,
}

impl Loadable<Vec<(Content, OxAgContentOptions)>> for OxAgContentPresets {
    fn load(&self) -> Vec<(Content, OxAgContentOptions)> {
        match self {
            OxAgContentPresets::None => NONE(),
            OxAgContentPresets::Default => DEFAULT(),
        }
    }
}

const NONE: fn() -> Vec<(Content, OxAgContentOptions)> = || {
    Content::iter()
        .map(|c| (c.to_default(), Default::default()))
        .collect()
};

const DEFAULT: fn() -> Vec<(Content, OxAgContentOptions)> = || {
    Vec::from([
        (
            Content::Rock(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 20,
                max_radius: 0,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.08,
            },
        ),
        (
            Content::Tree(0),
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 3,
                max_radius: 10,
                with_max_spawn_number: false,
                max_spawn_number: 100,
                percentage: 0.3,
            },
        ),
        (
            Content::Garbage(0),
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 2,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.03,
            },
        ),
        (
            Content::Fire,
            OxAgContentOptions {
                in_batches: true,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 2,
                with_max_spawn_number: true,
                max_spawn_number: 6,
                percentage: 0.04,
            },
        ),
        (
            Content::Coin(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 2,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.07,
            },
        ),
        (
            Content::Bin(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.01,
            },
        ),
        (
            Content::Crate(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 1,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.01,
            },
        ),
        (
            Content::Bank(0..0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 1,
                max_radius: 3,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.01,
            },
        ),
        (
            Content::Water(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 4,
                max_radius: 1,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 1.0,
            },
        ),
        (
            Content::Fish(0),
            OxAgContentOptions {
                in_batches: false,
                is_present: true,
                min_spawn_number: 0,
                max_radius: 1,
                with_max_spawn_number: false,
                max_spawn_number: 0,
                percentage: 0.2,
            },
        ),
    ])
};

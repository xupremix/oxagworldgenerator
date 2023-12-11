use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use robotics_lib::world::tile::TileType::{DeepWater, ShallowWater, Teleport};
use robotics_lib::world::tile::{Content, Tile, TileType};

pub(crate) fn spawn_circle(
    map: &mut Vec<Vec<Tile>>,
    rng: &mut StdRng,
    size: usize,
    center_x: usize,
    center_y: usize,
    radius: usize,
    target: &(Option<Content>, Option<TileType>),
) {
    let min_radius = radius.min(
        center_x
            .min(center_y)
            .min(size - center_x - 1)
            .min(size - center_y - 1),
    ) as isize;

    let mut x: isize = min_radius;
    let mut y: isize = 0;
    let mut decision = 1 - x; // Decision parameter to determine next point

    let center_x = center_x as isize;
    let center_y = center_y as isize;
    while x >= y {
        add(map, rng, center_x + x, center_y + y, target);
        add(map, rng, center_x + y, center_y + x, target);
        add(map, rng, center_x - y, center_y + x, target);
        add(map, rng, center_x - x, center_y + y, target);
        add(map, rng, center_x - x, center_y - y, target);
        add(map, rng, center_x - y, center_y - x, target);
        add(map, rng, center_x + y, center_y - x, target);
        add(map, rng, center_x + x, center_y - y, target);

        y += 1;
        if decision <= 0 {
            decision += 2 * y + 1;
        } else {
            x -= 1;
            decision += 2 * (y - x) + 1;
        }
    }

    // Fill the center of the circle
    for i in center_x - min_radius + 1..center_x + min_radius {
        for j in center_y - min_radius + 1..center_y + min_radius {
            if (i - center_x).pow(2) + (j - center_y).pow(2) <= min_radius.pow(2) {
                add(map, rng, i, j, target);
            }
        }
    }
}

fn add(
    map: &mut Vec<Vec<Tile>>,
    rng: &mut StdRng,
    row: isize,
    col: isize,
    target: &(Option<Content>, Option<TileType>),
) {
    match target {
        (Some(content), None) => {
            let mut value = 0;
            if content.properties().max() != 0 {
                value = rng.gen_range(0..content.properties().max());
            }
            let row = row as usize;
            let col = col as usize;
            if map[row][col].tile_type.properties().can_hold(content) {
                map[row][col].content = content.to_value(value);
            }
        }
        (None, Some(tile_type)) => {
            let row = row as usize;
            let col = col as usize;
            if ![ShallowWater, DeepWater, Teleport(false)].contains(&map[row][col].tile_type) {
                map[row][col].tile_type = *tile_type;
            }
        }
        _ => {}
    }
}

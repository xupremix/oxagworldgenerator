use crate::world_generator::spawning_tools::circle_spawn::spawn_circle;
use crate::world_generator::spawning_tools::F64MatData;
use crate::world_generator::tile_type_options::OxAgTileTypeOptions;
use rand::prelude::StdRng;
use rand::Rng;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType::{Grass, Hill, Lava};

impl F64MatData {
    pub(crate) fn lava_spawn(
        &self,
        map: &mut Vec<Vec<Tile>>,
        spawn_levels: &OxAgTileTypeOptions,
        rng: &mut StdRng,
    ) {
        for _ in 0..=rng.gen_range(spawn_levels.lava_n.clone()) {
            let (mut row, mut col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            while ![Grass, Hill].contains(&map[row][col].tile_type) {
                (row, col) = (rng.gen_range(0..self.size), rng.gen_range(0..self.size));
            }
            let radius = rng.gen_range(spawn_levels.lava_radius.clone());
            spawn_circle(map, rng, self.size, row, col, radius, &(None, Some(Lava)));
        }
    }
}

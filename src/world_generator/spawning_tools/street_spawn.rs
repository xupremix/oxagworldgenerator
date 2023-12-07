use crate::utils::constants::SAME_DIR_PROBABILITY;
use crate::world_generator::spawning_tools::F64MatData;
use rand::rngs::StdRng;
use rand::Rng;
use robotics_lib::world::tile::Tile;
use robotics_lib::world::tile::TileType::Street;

impl F64MatData {
    pub(crate) fn street_spawn(
        &mut self,
        map: &mut Vec<Vec<Tile>>,
        row: usize,
        col: usize,
        rng: &mut StdRng,
        prev_dir: (isize, isize),
        distance: usize,
    ) {
        if distance as isize - 1 < 0 || self.map[row][col].1 {
            return;
        }

        self.map[row][col].1 = true;
        map[row][col].tile_type = Street;

        let mut directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
        directions
            .to_vec()
            .retain(|&e| e != (-prev_dir.0, -prev_dir.1) && e != prev_dir);

        let new_row = row as isize + prev_dir.0;
        let new_col = col as isize + prev_dir.1;
        if rng.gen_bool(SAME_DIR_PROBABILITY)
            && 0 <= new_row
            && 0 <= new_col
            && new_row < self.size as isize
            && new_col < self.size as isize
            && !self.map[new_row as usize][new_col as usize].1
        {
            self.street_spawn(
                map,
                new_row as usize,
                new_col as usize,
                rng,
                prev_dir,
                distance - 1,
            );
            return;
        }

        let new_row = row as isize + directions[0].0;
        let new_col = col as isize + directions[0].1;
        if rng.gen_bool(0.5)
            && 0 <= new_row
            && 0 <= new_col
            && new_row < self.size as isize
            && new_col < self.size as isize
            && !self.map[new_row as usize][new_col as usize].1
        {
            self.street_spawn(
                map,
                new_row as usize,
                new_col as usize,
                rng,
                prev_dir,
                distance - 1,
            );
            return;
        }

        let new_row = row as isize + directions[1].0;
        let new_col = col as isize + directions[1].1;
        if 0 <= new_row
            && 0 <= new_col
            && new_row < self.size as isize
            && new_col < self.size as isize
            && !self.map[new_row as usize][new_col as usize].1
        {
            self.street_spawn(
                map,
                new_row as usize,
                new_col as usize,
                rng,
                prev_dir,
                distance - 1,
            );
        }
    }
}

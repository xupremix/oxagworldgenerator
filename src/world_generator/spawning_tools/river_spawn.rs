use crate::world_generator::spawning_tools::F64MatData;
use robotics_lib::world::tile::TileType::{DeepWater, Lava, ShallowWater};
use robotics_lib::world::tile::{Tile, TileType};
use std::collections::VecDeque;

impl F64MatData {
    pub(crate) fn river_spawn(&mut self, map: &mut Vec<Vec<Tile>>, row: usize, col: usize) -> bool {
        let mut directions = [(2.0, (-1, 0)), (2.0, (0, 1)), (2.0, (1, 0)), (2.0, (0, -1))];

        if row as i32 - 1 >= 0
            && !self.map[row - 1][col].1
            && !(map[row - 1][col].tile_type == Lava)
        {
            directions[0].0 = self.map[row - 1][col].0;
        }
        if col + 1 < self.size
            && !self.map[row][col + 1].1
            && !(map[row][col + 1].tile_type == Lava)
        {
            directions[1].0 = self.map[row][col + 1].0;
        }
        if row + 1 < self.size
            && !self.map[row + 1][col].1
            && !(map[row + 1][col].tile_type == Lava)
        {
            directions[2].0 = self.map[row + 1][col].0;
        }
        if col as i32 - 1 >= 0
            && !self.map[row][col - 1].1
            && !(map[row][col - 1].tile_type == Lava)
        {
            directions[3].0 = self.map[row][col - 1].0;
        }

        if directions.iter().all(|(v, _)| *v == 2.0) {
            return false;
        }

        if [ShallowWater, DeepWater].contains(&map[row][col].tile_type) {
            return true;
        }

        self.map[row][col].1 = true;
        map[row][col].tile_type = ShallowWater;

        directions.sort_unstable_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut directions = VecDeque::from(directions);

        while directions.len() > 0 {
            let (value, (row_offset, col_offset)) = directions.pop_front().unwrap();
            if value == 2.0 {
                return false;
            }
            let (new_row, new_col) = (
                (row as i32 + row_offset) as usize,
                (col as i32 + col_offset) as usize,
            );
            if self.river_spawn(map, new_row, new_col) {
                return true;
            }
        }
        return false;
    }
}

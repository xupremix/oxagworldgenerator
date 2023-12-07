use crate::world_generator::spawning_tools::F64MatData;
use robotics_lib::world::tile::{Tile, TileType};
use std::cmp::Ordering;
use std::collections::VecDeque;

impl F64MatData {
    pub(crate) fn line_spawn(
        &mut self,
        map: &mut Vec<Vec<Tile>>,
        tile: TileType,
        targets: &[TileType],
        row: usize,
        col: usize,
    ) -> bool {
        let mut directions = [(2.0, (-1, 0)), (2.0, (0, 1)), (2.0, (1, 0)), (2.0, (0, -1))];

        if row as i32 - 1 >= 0 && !self.map[row - 1][col].1 {
            directions[0].0 = self.map[row - 1][col].0;
        }
        if col + 1 < self.size && !self.map[row][col + 1].1 {
            directions[1].0 = self.map[row][col + 1].0;
        }
        if row + 1 < self.size && !self.map[row + 1][col].1 {
            directions[2].0 = self.map[row + 1][col].0;
        }
        if col as i32 - 1 >= 0 && !self.map[row][col - 1].1 {
            directions[3].0 = self.map[row][col - 1].0;
        }

        if directions.iter().all(|(v, _)| *v == 2.0) {
            return false;
        }

        if targets.contains(&map[row][col].tile_type) {
            return true;
        }

        self.map[row][col].1 = true;
        map[row][col].tile_type = tile.clone();

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
            if self.line_spawn(map, tile, targets, new_row, new_col) {
                return true;
            }
        }
        return false;
    }
}

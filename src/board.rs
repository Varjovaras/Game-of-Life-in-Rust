use crate::cell::{Cell, Status};

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: Vec<Vec<Cell>>,
}

impl Board {
    #[must_use]
    pub fn new(size: i32) -> Self {
        let squares: Vec<Vec<Cell>> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| Cell::new(i * size + j, Status::Dead))
                    .collect()
            })
            .collect();
        Self { squares }
    }

    #[must_use]
    pub fn new_with_every_fourth_alive(size: i32) -> Self {
        let squares: Vec<Vec<Cell>> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| {
                        let id = i * size + j;
                        let status = if id % 4 == 0 {
                            Status::Alive
                        } else {
                            Status::Dead
                        };
                        Cell::new(id, status)
                    })
                    .collect()
            })
            .collect();
        Self { squares }
    }

    #[must_use]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_possible_wrap)]
    pub fn get_adjacent_cells(&self, i: i32, j: i32) -> Vec<&Cell> {
        let mut adjacent_cells: Vec<&Cell> = Vec::new();
        if self.squares.is_empty() {
            return adjacent_cells;
        }

        let size = self.squares.len() as i32;
        for x in -1..=1 {
            for y in -1..=1 {
                let new_i = i + x;
                let new_j = j + y;
                if new_i >= 0 && new_i < size && new_j >= 0 && new_j < size && !(x == 0 && y == 0) {
                    adjacent_cells.push(&self.squares[new_i as usize][new_j as usize]);
                }
            }
        }
        adjacent_cells
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new_with_every_fourth_alive(16)
    }
}

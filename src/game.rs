use crate::{board::Board, rules::Rules};

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub rules: Rules,
    pub generation: i32,
}

impl Game {
    /// Creates a new [`Game`].
    ///
    /// # Panics
    ///
    /// Panics if `board_size` is less than 1.
    #[must_use]
    pub fn new(board_size: i32) -> Self {
        assert!(board_size >= 1, "Board size must be greater than 0");

        Self {
            board: Board::new_with_every_i_alive(board_size, 2),
            rules: Rules::default(),
            generation: 0,
        }
    }
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn next_generation(&self) -> Option<Self> {
        let mut new_self = self.clone();
        let mut new_board = self.board.clone();

        for (row_index, row) in self.board.squares.iter().enumerate() {
            for (cell_index, cell) in row.iter().enumerate() {
                let adjacent_squares = self
                    .board
                    .get_adjacent_cells(row_index as i32, cell_index as i32);
                let alive_cells = adjacent_squares
                    .iter()
                    .filter(|cell| cell.is_alive())
                    .count();

                if cell.is_dead() && alive_cells == self.rules.reproduction as usize {
                    new_board.squares[row_index][cell_index].revive();
                }

                if cell.is_alive()
                    && (alive_cells <= self.rules.underpopulation as usize
                        || alive_cells >= self.rules.overpopulation as usize)
                {
                    new_board.squares[row_index][cell_index].kill();
                }
            }
        }
        if new_self.board == new_board {
            return None;
        }
        new_self.board = new_board;
        new_self.generation += 1;
        Some(new_self)
    }

    #[must_use]
    pub fn all_dead(&self) -> bool {
        self.board
            .squares
            .iter()
            .flatten()
            .all(|cell: &crate::cell::Cell| cell.is_dead())
    }

    pub fn print_to_terminal(&self) {
        for row in &self.board.squares {
            for square in row {
                print!("{} ", square.status.current_status());
            }
            println!();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new(20)
    }
}

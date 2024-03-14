use crate::board::Board;

//node dies at underpopulation or 1
//node dies at overpopulation or more
//node lives at between underpopulation and underpopulation
//node resurrects at reproduction
#[derive(Debug, Clone)]
pub struct Rules {
    pub underpopulation: u8,
    pub overpopulation: u8,
    pub reproduction: u8,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            underpopulation: 1,
            overpopulation: 4,
            reproduction: 3,
        }
    }
}

impl Rules {
    #[must_use]
    pub const fn new(underpopulation: u8, overpopulation: u8, reproduction: u8) -> Self {
        Self {
            underpopulation,
            overpopulation,
            reproduction,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    pub board: Board,
    pub rules: Rules,
    pub generation: i32,
}

impl Game {
    #[must_use]
    pub fn new(board_size: i32) -> Self {
        Self {
            board: Board::new_with_every_fifth_alive(board_size),
            rules: Rules::default(),
            generation: 0,
        }
    }
    #[must_use]
    #[allow(clippy::cast_possible_wrap)]
    #[allow(clippy::cast_possible_truncation)]
    pub fn next_generation(&self) -> Self {
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
        new_self.board = new_board;
        new_self.generation += 1;
        new_self
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
        Self::new(16)
    }
}

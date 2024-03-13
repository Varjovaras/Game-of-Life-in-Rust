use crate::board::Board;

//node dies at underpopulation or 1
//node dies at overpopulation or more
//node lives at between underpopulation and underpopulation
//node resurrects at reproduction
#[derive(Debug)]
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

pub struct Game {
    pub board: Board,
    pub rules: Rules,
    pub generation: i32,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            rules: Rules::default(),
            generation: 0,
        }
    }

    pub fn print_to_terminal(&self) {
        for row in &self.board.squares {
            for square in row {
                print!("{} ", square.status.dead_or_alive());
            }
            println!();
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
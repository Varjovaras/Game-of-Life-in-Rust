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
    pub fn new() -> Self {
        Self {
            board: Board::default(),
            rules: Rules::default(),
            generation: 0,
        }
    }

    #[must_use]
    pub fn next_generation(&mut self) -> Self {
        //kill all
        for row in &mut self.board.squares {
            for square in row {
                square.kill();
            }
        }
        self.generation += 1;
        self.clone()
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
        Self::new()
    }
}

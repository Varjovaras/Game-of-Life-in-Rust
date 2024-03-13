use crate::square::Square;

#[derive(Debug)]
pub struct Board {
    pub squares: Vec<Vec<Square>>,
}

impl Board {
    #[must_use]
    pub fn new(size: usize) -> Self {
        let squares = vec![vec![Square::new(); size]; size];
        Self { squares }
    }

    // fn generation(&self) {}
}

impl Default for Board {
    fn default() -> Self {
        let squares = vec![vec![Square::new(); 16]; 16];
        Self { squares }
    }
}

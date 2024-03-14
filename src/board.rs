use crate::square::{Square, Status};

#[derive(Debug, Clone)]
pub struct Board {
    pub squares: Vec<Vec<Square>>,
}

impl Board {
    #[must_use]
    pub fn new(size: i32) -> Self {
        let squares: Vec<Vec<Square>> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| Square::new(i * size + j, Status::Dead))
                    .collect()
            })
            .collect();
        Self { squares }
    }

    #[must_use]
    fn new_with_every_fifth_alive(size: i32) -> Self {
        let squares: Vec<Vec<Square>> = (0..size)
            .map(|i| {
                (0..size)
                    .map(|j| {
                        let id = i * size + j;
                        let status = if id % 5 == 0 {
                            Status::Alive
                        } else {
                            Status::Dead
                        };
                        Square::new(id, status)
                    })
                    .collect()
            })
            .collect();
        Self { squares }
    }

    // fn generation(&self) {}
}

impl Default for Board {
    fn default() -> Self {
        Self::new_with_every_fifth_alive(16)
    }
}

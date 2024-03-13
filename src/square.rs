#[derive(Debug, Clone)]
pub enum Status {
    Alive,
    Dead,
}

impl Status {
    #[must_use]
    pub const fn dead_or_alive(&self) -> &str {
        match self {
            Status::Alive => "A",
            Status::Dead => "D",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Square {
    pub status: Status,
}

impl Square {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: Status::Dead,
        }
    }
}

impl Default for Square {
    fn default() -> Self {
        Self::new()
    }
}

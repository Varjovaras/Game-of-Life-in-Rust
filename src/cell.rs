#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Status {
    Alive,
    Dead,
}

impl Status {
    #[must_use]
    pub const fn current_status(&self) -> &str {
        match self {
            Self::Alive => "A",
            Self::Dead => "D",
        }
    }
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub status: Status,
    pub id: i32,
}

impl Cell {
    #[must_use]
    pub const fn new(id: i32, status: Status) -> Self {
        Self { status, id }
    }

    #[must_use]
    pub fn current_status(&self) -> String {
        self.status.current_status().into()
    }

    #[must_use]
    pub fn is_alive(&self) -> bool {
        self.status == Status::Alive
    }

    #[must_use]
    pub fn is_dead(&self) -> bool {
        self.status == Status::Dead
    }

    pub fn kill(&mut self) {
        self.status = Status::Dead;
    }

    pub fn revive(&mut self) {
        self.status = Status::Alive;
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self::new(0, Status::Dead)
    }
}

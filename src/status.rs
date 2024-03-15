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
            Self::Dead => "-",
        }
    }
}

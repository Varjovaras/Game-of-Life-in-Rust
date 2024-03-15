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

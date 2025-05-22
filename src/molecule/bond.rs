use super::Parity;

#[derive(Debug, PartialEq)]
pub struct Bond {
    // Having electrons as f32 allows for fractional bonds
    pub electrons: f32,
    pub parity: Option<Parity>,
    pub tid: usize,
}

impl Bond {
    pub fn new(electrons: f32, parity: Option<Parity>, tid: usize) -> Self {
        Self {
            electrons,
            parity,
            tid,
        }
    }

    pub fn order(&self) -> f32 {
        self.electrons as f32 / 2 as f32
    }
}

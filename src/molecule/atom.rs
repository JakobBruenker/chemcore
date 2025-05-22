use super::{Element, Parity};

#[derive(Debug, PartialEq, Clone)]
pub struct Atom {
    pub element: Element,
    pub isotope: Option<u16>,
    pub electrons: u8,
    pub parity: Option<Parity>,
    pub hydrogens: u8,
}

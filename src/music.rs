mod scale;
mod transport;

pub use scale::*;
pub use transport::*;

pub type NoteMask = [Option<Note>; 12];

pub struct TimeSignature {
    pub top: usize,
    pub bottom: usize,
}

impl From<(usize, usize)> for TimeSignature {
    fn from(value: (usize, usize)) -> Self {
        Self {
            top: value.0,
            bottom: value.1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Note {
    A(Acc),
    B(Acc),
    C(Acc),
    D(Acc),
    E(Acc),
    F(Acc),
    G(Acc),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Acc {
    Flt,
    Nat,
    Shp,
}

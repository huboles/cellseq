mod scale;
mod transport;

pub use scale::*;
pub use transport::*;

use std::{fmt::Display, ops::Deref};

#[derive(Clone, Copy, Debug)]
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
    Off,
    A(Acc),
    B(Acc),
    C(Acc),
    D(Acc),
    E(Acc),
    F(Acc),
    G(Acc),
}

impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Note::Off => "".to_string(),
            Note::A(a) => format!("A{a}"),
            Note::B(a) => format!("B{a}"),
            Note::C(a) => format!("C{a}"),
            Note::D(a) => format!("D{a}"),
            Note::E(a) => format!("E{a}"),
            Note::F(a) => format!("F{a}"),
            Note::G(a) => format!("G{a}"),
        };

        write!(f, "{str}")
    }
}

impl Note {
    pub fn to_char(&self) -> char {
        match self {
            Note::Off => ' ',
            Note::A(_) => 'a',
            Note::B(_) => 'b',
            Note::C(_) => 'c',
            Note::D(_) => 'd',
            Note::E(_) => 'e',
            Note::F(_) => 'f',
            Note::G(_) => 'g',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Acc {
    Flt,
    Nat,
    Shp,
}

impl Display for Acc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Acc::Flt => "b",
            Acc::Nat => "",
            Acc::Shp => "#",
        };

        write!(f, "{str}")
    }
}

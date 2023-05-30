use crate::Point;
use crossterm::style::{
    Attribute, Attributes,
    Color::{Black, Grey, White},
    Colors,
};
use ndarray::Array2;
use std::ops::Deref;

use super::*;

pub trait Map<T> {
    fn try_point(&self, point: Point) -> bool;
    fn get_point(&self, point: Point) -> Option<T>;
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn characters(&self) -> (char, char);
    fn on_colors(&self) -> Colors;
    fn off_colors(&self) -> Colors;
    fn styles(&self) -> (Attributes, Attributes);
    fn update(&mut self);
}

#[derive(Debug, Clone)]
pub struct Mask {
    pub mask: Array2<Option<Note>>,
}

impl Mask {
    pub fn new(x: usize, y: usize) -> Self {
        let mask = Array2::from_elem((y, x), None);
        Self { mask }
    }
}

impl Deref for Mask {
    type Target = Array2<Option<Note>>;
    fn deref(&self) -> &Self::Target {
        &self.mask
    }
}

impl Map<Option<Note>> for Mask {
    fn x_size(&self) -> usize {
        self.ncols()
    }

    fn y_size(&self) -> usize {
        self.nrows()
    }

    fn try_point(&self, point: Point) -> bool {
        self.get((point.y, point.x)).is_some()
    }

    fn get_point(&self, point: Point) -> Option<Option<Note>> {
        self.get((point.y, point.x)).copied()
    }

    fn characters(&self) -> (char, char) {
        ('■', '□')
    }

    fn on_colors(&self) -> Colors {
        Colors::new(White, Black)
    }

    fn off_colors(&self) -> Colors {
        Colors::new(Grey, Black)
    }

    fn styles(&self) -> (Attributes, Attributes) {
        let on = Attributes::from(Attribute::Bold);
        let off = Attributes::from(Attribute::Reset);
        (on, off)
    }

    fn update(&mut self) {}
}

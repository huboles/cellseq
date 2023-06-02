use crate::Point;
use crossterm::style::{
    Attribute, Attributes,
    Color::{Black, White},
    Colors,
};
use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::ops::{Deref, DerefMut};

use super::*;

pub trait Map<T> {
    fn try_point(&self, point: Point) -> bool;
    fn get_point(&self, point: Point) -> Option<T>;
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn characters(&self) -> (char, char);
    fn colors(&self) -> (Colors, Colors);
    fn styles(&self) -> (Attributes, Attributes);
    fn update(&mut self);
}

#[derive(Debug, Clone)]
pub struct Mask {
    pub mask: Array2<Note>,
    pub colors: (Colors, Colors),
}

impl Mask {
    pub fn new(area: Area) -> Self {
        let mask = Array2::from_elem((area.width(), area.height()), Note::Off);
        Self {
            mask,
            colors: (Colors::new(White, Black), Colors::new(Black, Black)),
        }
    }

    pub fn randomize(&mut self, val: f64, scale: Scale) {
        let mut rng = thread_rng();
        for f in self.iter_mut() {
            if rng.gen::<f64>() > val {
                let notes = scale.to_notes();
                *f = notes[rng.gen::<usize>() % notes.len()];
            }
        }
    }
}

impl Deref for Mask {
    type Target = Array2<Note>;
    fn deref(&self) -> &Self::Target {
        &self.mask
    }
}

impl DerefMut for Mask {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mask
    }
}

impl Map<Note> for Mask {
    fn x_size(&self) -> usize {
        self.ncols()
    }

    fn y_size(&self) -> usize {
        self.nrows()
    }

    fn try_point(&self, point: Point) -> bool {
        if let Some(note) = self.get((point.y, point.x)) {
            *note != Note::Off
        } else {
            false
        }
    }

    fn get_point(&self, point: Point) -> Option<Note> {
        self.get((point.y, point.x)).copied()
    }

    fn characters(&self) -> (char, char) {
        ('■', '□')
    }

    fn colors(&self) -> (Colors, Colors) {
        self.colors
    }

    fn styles(&self) -> (Attributes, Attributes) {
        let on = Attributes::from(Attribute::Bold);
        let off = Attributes::from(Attribute::Reset);
        (on, off)
    }

    fn update(&mut self) {}
}

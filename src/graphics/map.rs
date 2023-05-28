use crate::Point;
use ndarray::Array2;
use std::ops::Deref;

pub trait Map<T> {
    fn try_point(&self, point: Point) -> bool;
    fn get_point(&self, point: Point) -> Option<T>;
    fn x_size(&self) -> usize;
    fn y_size(&self) -> usize;
    fn graphics(&self) -> (char, char);
    fn update(&mut self);
}

#[derive(Debug, Clone)]
pub struct Mask<T: Clone> {
    pub mask: Array2<T>,
}

impl<T: Clone> Mask<T> {
    pub fn new(x: usize, y: usize, default: T) -> Self {
        let mask = Array2::from_elem((y, x), default);
        Self { mask }
    }
}

impl<T: Clone> Deref for Mask<T> {
    type Target = Array2<T>;
    fn deref(&self) -> &Self::Target {
        &self.mask
    }
}

impl<T: Clone> Map<T> for Mask<T> {
    fn x_size(&self) -> usize {
        self.ncols()
    }

    fn y_size(&self) -> usize {
        self.nrows()
    }

    fn try_point(&self, point: Point) -> bool {
        self.get((point.y, point.x)).is_some()
    }

    fn get_point(&self, point: Point) -> Option<T> {
        self.get((point.y, point.x)).cloned()
    }

    fn graphics(&self) -> (char, char) {
        ('■', '□')
    }

    fn update(&mut self) {}
}

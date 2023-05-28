use crate::{Map, Mask, Point};
use crossterm::style::{Attribute, Color};
use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::{cell::Cell, ops::Deref};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Clone, Debug)]
pub struct World {
    pub map: Mask<Cell<State>>,
}

impl World {
    pub fn new(x: usize, y: usize) -> Self {
        let map = Mask::new(x, y, Cell::from(State::Dead));
        Self { map }
    }

    pub fn randomize(&mut self, val: f64) {
        let mut rng = thread_rng();
        for cell in self.map.iter() {
            if rng.gen::<f64>() > val {
                cell.set(State::Alive)
            } else {
                cell.set(State::Dead)
            }
        }
    }

    fn wrap_walls(&mut self) {
        for (bottom, top) in self.row(self.y_size() - 1).iter().zip(self.row(0).iter()) {
            bottom.set(State::Dead);
            top.set(State::Dead);
        }

        for (right, left) in self
            .column(self.x_size() - 1)
            .iter()
            .zip(self.column(0).iter())
        {
            right.set(State::Dead);
            left.set(State::Dead);
        }
    }
}

impl Map<Cell<State>> for World {
    fn update(&mut self) {
        let mut previous = self.clone();
        previous.wrap_walls();
        for (next, prev) in self
            .windows((3, 3))
            .into_iter()
            .zip(previous.windows((3, 3)))
        {
            let count = prev.iter().filter(|x| x.get() == State::Alive).count();

            let cell = next.get((1, 1)).unwrap();

            match cell.get() {
                State::Dead => {
                    if count == 3 {
                        cell.set(State::Alive);
                    }
                }
                State::Alive => {
                    if count < 3 || count > 4 {
                        cell.set(State::Dead);
                    }
                }
            }
        }
    }

    fn x_size(&self) -> usize {
        self.ncols()
    }

    fn y_size(&self) -> usize {
        self.nrows()
    }

    fn characters(&self) -> (char, char) {
        ('●', '◌')
    }

    fn fg_colors(&self) -> (Color, Color) {
        (Color::Green, Color::Grey)
    }

    fn bg_colors(&self) -> (Color, Color) {
        (Color::Black, Color::Black)
    }

    fn styles(&self) -> (Attribute, Attribute) {
        (Attribute::Bold, Attribute::Reset)
    }

    fn try_point(&self, point: Point) -> bool {
        if let Some(cell) = self.get((point.y, point.x)) {
            if cell.get() == State::Alive {
                return true;
            }
        }
        false
    }
    fn get_point(&self, point: Point) -> Option<Cell<State>> {
        self.get((point.y, point.x)).cloned()
    }
}

impl Deref for World {
    type Target = Array2<Cell<State>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

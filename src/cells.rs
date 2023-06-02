use crate::{Map, Point};
use crossterm::style::{
    Attribute, Attributes,
    Color::{Black, Green, Grey},
    Colors,
};
use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::{cell::Cell, ops::Deref};

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Debug, Clone)]
pub struct World {
    pub map: Array2<Cell<State>>,
}

impl World {
    pub fn new(area: Area) -> Self {
        let map = Array2::from_elem((area.width(), area.height()), Cell::new(State::Dead));
        Self { map }
    }

    pub fn randomize(&mut self, val: f64) {
        let mut rng = thread_rng();
        for cell in self.map.iter() {
            if rng.gen::<f64>() > val {
                cell.set(State::Alive);
            } else {
                cell.set(State::Dead);
            }
        }
    }

    fn wrap_walls(&mut self) {
        macro_rules! wrap {
            ($portal:expr,$wall:expr) => {
                for (portal, wall) in $portal.iter().zip($wall) {
                    portal.set(wall.get());
                }
            };
        }

        wrap!(self.column(0), self.column(self.x_size() - 2));
        wrap!(self.column(self.x_size() - 1), self.column(1));
        wrap!(self.row(0), self.row(self.y_size() - 2));
        wrap!(self.row(self.y_size() - 1), self.row(1));
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
                    if !(3..=4).contains(&count) {
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

    fn colors(&self) -> (Colors, Colors) {
        (Colors::new(Green, Black), Colors::new(Grey, Black))
    }

    fn styles(&self) -> (Attributes, Attributes) {
        (Attribute::Bold.into(), Attribute::Reset.into())
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

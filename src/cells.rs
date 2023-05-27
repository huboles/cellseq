use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::{cell::Cell, fmt::Display, ops::Deref};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Map {
    pub map: Array2<Cell<State>>,
    pub rows: usize,
    pub cols: usize,
}

impl Map {
    pub fn new(x: usize, y: usize) -> Self {
        let map = Array2::from_elem((y, x), Cell::new(State::Dead));
        Self {
            map,
            cols: x,
            rows: y,
        }
    }

    pub fn update(&mut self) {
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
        for (bottom, top) in self.row(self.rows - 1).iter().zip(self.row(0).iter()) {
            bottom.set(State::Dead);
            top.set(State::Dead);
        }

        for (right, left) in self.column(self.cols - 1).iter().zip(self.column(0).iter()) {
            right.set(State::Dead);
            left.set(State::Dead);
        }
    }
}

impl Deref for Map {
    type Target = Array2<Cell<State>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

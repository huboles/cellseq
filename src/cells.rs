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
    pub fn new(rows: usize, cols: usize) -> Self {
        let map = Array2::from_elem((rows, cols), Cell::new(State::Dead));
        Self { map, rows, cols }
    }

    pub fn update(&mut self) {
        let previous = self.clone();
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
                    if count < 2 || count > 3 {
                        cell.set(State::Dead);
                    }
                }
            }
        }
    }

    pub fn randomize(&mut self, val: f64) {
        let mut rng = thread_rng();
        for cell in self.map.iter() {
            if rng.gen::<f64>() < val {
                cell.set(State::Alive)
            } else {
                cell.set(State::Dead)
            }
        }

        let walls = vec![
            self.row(0),
            self.row(self.rows - 1),
            self.column(0),
            self.column(self.cols - 1),
        ];

        for wall in walls {
            for cell in wall.iter() {
                cell.set(State::Dead)
            }
        }
    }
}

impl Deref for Map {
    type Target = Array2<Cell<State>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let map: String = self
            .outer_iter()
            .into_iter()
            .map(|row| {
                row.iter()
                    .map(|x| match x.get() {
                        State::Dead => ' ',
                        State::Alive => 'o',
                    })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");

        write!(f, "{map}")
    }
}

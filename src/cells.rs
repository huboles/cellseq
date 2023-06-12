use crossterm::{
    cursor::MoveTo,
    queue,
    style::{
        Attribute::{Bold, Reset},
        Color::{Black, Green, Grey},
        ContentStyle, Print, SetStyle,
    },
};
use ndarray::Array2;
use rand::{thread_rng, Rng};
use std::{cell::Cell, io::stdout, ops::Deref};

use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum State {
    Dead,
    Alive,
}

#[derive(Debug, Clone)]
pub struct World {
    pub map: Array2<Cell<State>>,
    pub area: Area,
}

impl World {
    pub fn new(area: Area) -> Self {
        let map = Array2::from_elem((area.width(), area.height()), Cell::new(State::Dead));
        Self { map, area }
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

        wrap!(self.column(0), self.column(self.area.width() - 2));
        wrap!(self.column(self.area.width() - 1), self.column(1));
        wrap!(self.row(0), self.row(self.area.height() - 2));
        wrap!(self.row(self.area.height() - 1), self.row(1));
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
                    if !(3..=4).contains(&count) {
                        cell.set(State::Dead);
                    }
                }
            }
        }
    }

    fn draw_point(&self, point: Point, offset: Point) -> Result<()> {
        let mut style = ContentStyle {
            foreground_color: None,
            background_color: Some(Black),
            underline_color: None,
            attributes: Reset.into(),
        };
        let char = if let Some(cell) = self.get::<(usize, usize)>(point.into()) {
            if cell.get() == State::Alive {
                style.foreground_color = Some(Green);
                style.attributes = Bold.into();
                '●'
            } else {
                style.background_color = Some(Grey);
                '◌'
            }
        } else {
            ' '
        };

        queue!(
            stdout(),
            MoveTo(offset.x, offset.y),
            SetStyle(style),
            Print(char)
        )?;
        Ok(())
    }
}

impl Deref for World {
    type Target = Array2<Cell<State>>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

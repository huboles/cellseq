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
use std::{
    cell::Cell,
    io::{stdout, Write},
    ops::Deref,
};
use tokio::sync::watch::Receiver;

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
        let width = area.width().try_into().unwrap_or(0);
        let height = area.height().try_into().unwrap_or(0);

        let map = Array2::from_elem((height + 1, width + 1), Cell::new(State::Dead));
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
        for (hidden, shown) in self
            .column(0)
            .iter()
            .zip(self.column((self.area.width() - 2).try_into().unwrap_or(0)))
        {
            hidden.set(shown.get());
        }

        for (hidden, shown) in self
            .column((self.area.width() - 1).try_into().unwrap_or(0))
            .iter()
            .zip(self.column(1))
        {
            hidden.set(shown.get());
        }

        for (hidden, shown) in self
            .row(0)
            .iter()
            .zip(self.row((self.area.height() - 2).try_into().unwrap_or(0)))
        {
            hidden.set(shown.get());
        }

        for (hidden, shown) in self
            .row((self.area.height() - 1).try_into().unwrap_or(0))
            .iter()
            .zip(self.row(1))
        {
            hidden.set(shown.get());
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
                    if !(3..=4).contains(&count) {
                        cell.set(State::Dead);
                    }
                }
            }
        }
    }

    pub fn draw_point(&self, cell: Point) -> Result<()> {
        let mut style = ContentStyle {
            foreground_color: None,
            background_color: Some(Black),
            underline_color: None,
            attributes: Reset.into(),
        };
        let (col, row) = cell.into();
        let char = if let Some(cell) = self.get((row, col)) {
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
            MoveTo(cell.x + self.area.origin.x, cell.y + self.area.origin.y),
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

pub async fn run_world(clock: &mut Receiver<usize>, world: &mut World) -> Result<()> {
    loop {
        world.update();

        for x in 1..(world.area.width() - 1).try_into()? {
            for y in 1..(world.area.height()).try_into()? {
                world.draw_point(Point::new(x, y))?;
            }
        }

        clock.changed().await?;

        stdout().flush()?;
    }
}

use crate::{Map, State};
use crossterm::{
    cursor,
    event::{self, read, Event},
    execute, queue,
    style::{self, Stylize},
    terminal, Result,
};
use ndarray::Array2;
use std::{
    io::{stdout, Write},
    ops::Deref,
    thread,
    time::Duration,
};

pub type Origin = (usize, usize);

#[derive(Clone, Debug)]
pub struct Mask<T: Clone> {
    pub cols: usize,
    pub rows: usize,
    pub mask: Array2<Option<T>>,
}

impl<T: Clone> Mask<T> {
    pub fn new(x: usize, y: usize) -> Self {
        let mask = Array2::from_elem((y, x), None);
        Self {
            cols: x,
            rows: y,
            mask,
        }
    }
}

impl<T: Clone> Deref for Mask<T> {
    type Target = Array2<Option<T>>;
    fn deref(&self) -> &Self::Target {
        &self.mask
    }
}

pub fn edit_mask<T: Clone>(mask: &Mask<T>, origin: Origin) -> Result<()> {
    todo!()
}

pub fn draw_mask<T: Clone>(mask: &Mask<T>, origin: Origin) -> Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
    for x in 1..(mask.cols - 1) {
        for y in 1..(mask.rows - 1) {
            let x_off: u16 = (x + origin.0).try_into().unwrap();
            let y_off: u16 = (y + origin.1).try_into().unwrap();
            if mask.get((y, x)).unwrap().is_some() {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent("■".green())
                )?;
            } else {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent("□".grey())
                )?;
            }
        }
    }
    stdout().flush()
}

pub fn draw_frame(map: &Map, origin: Origin) -> Result<()> {
    for x in 1..(map.cols - 1) {
        for y in 1..(map.rows - 1) {
            let x_off: u16 = (x + origin.0).try_into().unwrap();
            let y_off: u16 = (y + origin.1).try_into().unwrap();
            if map.get((y, x)).unwrap().get() == State::Alive {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent("●".green())
                )?;
            } else {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent("◌".grey())
                )?;
            }
        }
    }
    stdout().flush()
}

pub fn run_map(map: &mut Map, origin: Origin, time: Duration) -> Result<()> {
    loop {
        map.update();

        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        draw_frame(&map, origin)?;

        thread::sleep(time);
    }
}

pub fn loop_map(map: &mut Map, origin: (usize, usize), time: Duration, steps: usize) -> Result<()> {
    loop {
        let mut tmp = map.clone();
        for _ in 0..steps {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            draw_frame(&tmp, origin)?;
            tmp.update();
            thread::sleep(time);
        }
    }
}

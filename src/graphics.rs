mod map;
mod point;

pub use map::*;
pub use point::*;

use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal, Result,
};
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

pub struct Cursor {}

pub fn draw_frame<T>(map: &mut impl Map<T>, offset: Point) -> Result<()> {
    let (on, off) = map.graphics();
    for x in 1..(map.x_size() - 1) {
        for y in 1..(map.y_size() - 1) {
            let point = Point::new(x, y);
            let offset = point + offset;
            let x_off: u16 = offset.x.try_into().unwrap();
            let y_off: u16 = offset.y.try_into().unwrap();
            if map.try_point(point) {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent(on.green())
                )?;
            } else {
                queue!(
                    stdout(),
                    cursor::Hide,
                    cursor::MoveTo(x_off, y_off),
                    style::PrintStyledContent(off.grey())
                )?;
            }
        }
    }
    stdout().flush()
}

pub fn run_map<T>(map: &mut impl Map<T>, offset: Point, time: Duration) -> Result<()> {
    loop {
        map.update();

        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        draw_frame(map, offset)?;

        thread::sleep(time);
    }
}

pub fn loop_map<T>(
    map: &mut (impl Map<T> + Clone),
    offset: Point,
    time: Duration,
    steps: usize,
) -> Result<()> {
    loop {
        let mut tmp = map.clone();
        for _ in 0..steps {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            draw_frame(&mut tmp, offset)?;
            tmp.update();
            thread::sleep(time);
        }
    }
}

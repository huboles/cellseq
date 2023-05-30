mod actions;
mod area;
mod keys;
mod layout;
mod map;
mod point;
mod selector;

pub use actions::*;
pub use area::*;
pub use keys::*;
pub use layout::*;
pub use map::*;
pub use point::*;
pub use selector::*;

use crossterm::{
    cursor::{Hide, MoveTo},
    execute, queue,
    style::{Print, SetAttributes, SetColors},
    terminal,
};
use eyre::Result;
use std::{
    io::{stdout, Write},
    thread,
    time::Duration,
};

pub struct Cursor {}

pub fn draw_frame<T>(map: &mut impl Map<T>, offset: Point) -> Result<()> {
    let (char_on, char_off) = map.characters();
    let on_colors = map.on_colors();
    let off_colors = map.off_colors();
    let (style_on, style_off) = map.styles();

    for x in 1..(map.x_size() - 1) {
        for y in 1..(map.y_size() - 1) {
            let point = Point::new(x, y);
            let (x_off, y_off) = point.u16_offset(offset)?;

            if map.try_point(point) {
                queue!(
                    stdout(),
                    Hide,
                    MoveTo(x_off, y_off),
                    SetAttributes(style_on),
                    SetColors(on_colors),
                    Print(char_on)
                )?;
            } else {
                queue!(
                    stdout(),
                    Hide,
                    MoveTo(x_off, y_off),
                    SetAttributes(style_off),
                    SetColors(off_colors),
                    Print(char_off)
                )?;
            }
        }
    }
    Ok(())
}

pub fn run_map<T>(map: &mut impl Map<T>, offset: Point, time: Duration) -> Result<()> {
    loop {
        map.update();

        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        draw_frame(map, offset)?;

        stdout().flush()?;

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
            stdout().flush()?;
            tmp.update();
            thread::sleep(time);
        }
    }
}

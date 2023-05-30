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

use super::*;

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

pub fn render_loop(state: &mut GlobalState) -> Result<()> {
    execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

    state.layout.draw_outlines()?;

    loop {
        let tick = state.transport.tick;
        let timer = std::thread::spawn(move || thread::sleep(tick));

        if !state.transport.running {
            timer.join().unwrap();
            continue;
        }

        state.world.update();

        draw_map(&mut state.world, &state.layout.cells)?;
        // draw_map(&mut state.mask[0], &state.layout.mask)?;

        timer.join().unwrap();

        stdout().flush()?;
    }
}

pub fn draw_map<T>(map: &mut impl Map<T>, area: &Area) -> Result<()> {
    let ((x_zero, y_zero), (x_max, y_max)) = area.to_u16()?;

    let origin = Point::new(x_zero.into(), y_zero.into());

    let (char_on, char_off) = map.characters();
    let on_colors = map.on_colors();
    let off_colors = map.off_colors();
    let (style_on, style_off) = map.styles();

    for x in 0..=(map.x_size()) {
        for y in 0..=(map.y_size()) {
            let point = Point::new(x, y);
            let (x_off, y_off) = origin.u16_offset(point)?;

            if x_off <= x_zero || x_off >= x_max || y_off <= y_zero || y_off >= y_max {
                continue;
            }

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

    area.outline_area()?;

    Ok(())
}

pub fn run_map<T>(map: &mut impl Map<T>, area: &Area, time: Duration) -> Result<()> {
    loop {
        map.update();

        execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;

        draw_map(map, area)?;

        stdout().flush()?;

        thread::sleep(time);
    }
}

pub fn loop_map<T>(
    map: &mut (impl Map<T> + Clone),
    area: &Area,
    time: Duration,
    steps: usize,
) -> Result<()> {
    loop {
        let mut tmp = map.clone();
        for _ in 0..steps {
            execute!(stdout(), terminal::Clear(terminal::ClearType::All))?;
            draw_map(&mut tmp, area)?;
            stdout().flush()?;
            tmp.update();
            thread::sleep(time);
        }
    }
}

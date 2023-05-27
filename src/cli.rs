use crate::{Map, State};
use crossterm::{
    cursor, execute, queue,
    style::{self, Stylize},
    terminal, Result,
};
use std::io::{stdout, Write};

pub fn draw_frame(map: &Map, origin: (usize, usize)) -> Result<()> {
    for y in 1..(map.cols - 1) {
        for x in 1..(map.rows - 1) {
            let x_off: u16 = (x + origin.0).try_into().unwrap();
            let y_off: u16 = (y + origin.1).try_into().unwrap();
            if map.get((x, y)).unwrap().get() == State::Alive {
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

pub fn run_map(map: &mut Map, origin: (usize, usize), time: usize) -> Result<()> {
    let mut stdout = stdout();
    loop {
        map.update();

        execute!(stdout, terminal::Clear(terminal::ClearType::All))?;

        draw_frame(&map, origin)?;

        std::thread::sleep(std::time::Duration::from_millis(time.try_into().unwrap()));
    }
}

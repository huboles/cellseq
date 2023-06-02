mod cells;
mod graphics;
mod music;
mod state;

pub use cells::*;
pub use graphics::*;
pub use music::*;
pub use state::*;

use eyre::Result;
use std::time::Duration;

pub fn exit() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::cursor::Show)?;
    std::process::exit(0);
}

pub fn bpm_to_ms(bpm: usize) -> Duration {
    let ms = 60000 / bpm;
    Duration::from_millis(ms.try_into().unwrap())
}

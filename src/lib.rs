mod cells;
mod graphics;
mod music;

pub use cells::*;
pub use graphics::*;
pub use music::*;

use eyre::Result;
use std::time::Duration;

pub fn exit() -> Result<()> {
    crossterm::terminal::disable_raw_mode()?;
    std::process::exit(0);
}

pub fn bpm_to_ms(bpm: usize) -> Duration {
    let ms = 60000 / bpm;
    Duration::from_millis(ms.try_into().unwrap())
}

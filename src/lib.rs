mod cells;
mod graphics;
mod music;

pub use cells::*;
pub use graphics::*;
pub use music::*;

use std::time::Duration;

pub fn bpm_to_ms(bpm: usize) -> Duration {
    let ms = 60000 / bpm;
    Duration::from_millis(ms.try_into().unwrap())
}

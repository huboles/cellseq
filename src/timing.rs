use super::*;

use std::time::Duration;
use tokio::{
    sync::watch::Sender,
    time::{interval, Interval},
};

pub struct Metronome {
    pub interval: Interval,
    pub tick_len: Duration,
    pub bpm: usize,
    pub clock: Sender<usize>,
    pub tick: usize,
}

impl Metronome {
    pub fn new(clock: Sender<usize>, bpm: usize) -> Self {
        let tick_len = Duration::from_millis((60000 / bpm).try_into().unwrap_or(0));
        let interval = interval(tick_len);
        Self {
            interval,
            tick_len,
            bpm,
            clock,
            tick: 0,
        }
    }
}

pub async fn run_clock(metro: &mut Metronome) -> Result<()> {
    loop {
        metro.interval.tick().await;
        metro.clock.send(metro.tick)?;
        metro.tick += 1;
    }
}

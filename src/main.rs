use std::{io::Write, thread::JoinHandle};

use alsa::{
    rawmidi::{Rawmidi, IO},
    Direction,
};
use cellseq::*;

use iced::{window, Application, Settings};

use eyre::{eyre, Result};
use tokio::sync::mpsc::channel;

pub fn main() -> Result<()> {
    let (midi_snd, mut midi_rcv) = channel::<u8>(256);
    let midi = MidiLink::new(midi_snd);

    let midi_sink = Rawmidi::new("virtual", Direction::Playback, false)?;

    let midi_loop: JoinHandle<Result<()>> = std::thread::spawn(move || {
        let mut midi_io = midi_sink.io();
        while let Some(byte) = midi_rcv.blocking_recv() {
            midi_io.write_all(&[byte])?;
        }
        Ok(())
    });

    // running the graphics window
    CellSeq::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        flags: midi,
        ..Settings::default()
    })?;

    midi_loop.join().map_err(|_| eyre!("join failure"))??;

    Ok(())
}

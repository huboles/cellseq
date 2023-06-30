use cellseq::*;

use iced::{window, Application, Settings};

use eyre::Result;
use tokio::sync::mpsc::channel;

pub fn main() -> Result<()> {
    let (midi_snd, _midi_rcv) = channel::<Option<u8>>(256);

    let midi = MidiLink::new(midi_snd);

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

    Ok(())
}

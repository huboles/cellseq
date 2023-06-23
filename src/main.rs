use cellseq::*;

use iced::{window, Application, Settings};

use eyre::Result;
use jack::{Client, ClientOptions, ClosureProcessHandler, Control, MidiOut, ProcessScope, RawMidi};
use tokio::sync::mpsc::{channel, Receiver, Sender};

pub fn main() -> Result<()> {
    let (midi_snd, midi_rcv) = channel::<u8>(256);

    // setting up jack client
    let (jack_client, jack_status) = Client::new("cellseq", ClientOptions::empty())?;
    let mut midi_port = jack_client.register_port("cellseq_midi", MidiOut::default())?;

    let process_handler = ClosureProcessHandler::new(move |_: &Client, scope: &ProcessScope| {
        let writer = midi_port.writer(scope);

        Control::Continue
    });

    let jack = jack_client.activate_async((), process_handler)?;

    // running the graphics window
    CellSeq::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
    .map_err(|_| Ok(()));

    jack.deactivate()?;

    Ok(())
}

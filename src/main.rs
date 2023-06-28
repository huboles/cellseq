use cellseq::*;

use iced::{window, Application, Settings};

use eyre::Result;
use jack::{Client, ClientOptions, ClosureProcessHandler, Control, MidiOut, ProcessScope, RawMidi};
use tokio::sync::mpsc::channel;

pub fn main() -> Result<()> {
    let (midi_snd, mut midi_rcv) = channel::<Option<u8>>(256);

    // setting up jack client
    // let (jack_client, _status) = Client::new("cellseq", ClientOptions::empty())?;
    // let mut midi_port = jack_client.register_port("cellseq_midi", MidiOut::default())?;

    // let process_handler = ClosureProcessHandler::new(move |_: &Client, scope: &ProcessScope| {
    //     let mut writer = midi_port.writer(scope);
    //     let mut bytes = Vec::new();

    //     while let Ok(Some(byte)) = midi_rcv.try_recv() {
    //         bytes.push(byte);
    //     }

    //     let time = scope.frames_since_cycle_start();
    //     writer
    //         .write(&RawMidi {
    //             time,
    //             bytes: &bytes[0..bytes.len()],
    //         })
    //         .unwrap();

    //     Control::Continue
    // });

    // let jack = jack_client.activate_async((), process_handler)?;

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

    // jack.deactivate()?;

    Ok(())
}

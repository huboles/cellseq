use cellseq::*;

use iced::{window, Application, Settings};

pub fn main() -> iced::Result {
    CellSeq::run(Settings {
        antialiasing: true,
        window: window::Settings {
            position: window::Position::Centered,
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

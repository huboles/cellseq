use std::{collections::VecDeque, fmt::Display};

use thiserror::Error;
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct MidiLink {
    buffer: VecDeque<u8>,
    channel: Sender<Option<u8>>,
}

impl Default for MidiLink {
    fn default() -> Self {
        let (send, _) = tokio::sync::mpsc::channel(128);
        Self {
            buffer: VecDeque::default(),
            channel: send,
        }
    }
}

impl<'a> MidiLink {
    pub fn new(channel: Sender<Option<u8>>) -> Self {
        Self {
            buffer: VecDeque::default(),
            channel,
        }
    }

    pub fn update(&mut self, message: MidiMessage) {
        let bytes = message.as_bytes().unwrap();

        for byte in bytes.iter().filter_map(|x| *x) {
            self.buffer.push_back(byte);
        }
    }

    pub async fn tick(&mut self) {
        for byte in self.buffer.iter() {
            self.channel.send(Some(*byte)).await.unwrap();
        }

        self.channel.send(None).await.unwrap();
        self.buffer.clear();
    }
}

#[derive(Clone, Copy, Debug, Error)]
pub enum MidiError {
    #[error("value greater than 127: {message}")]
    ValueOverflow { message: MidiMessage },
    #[error("channel not within (0-15): {message}")]
    ChannelOverflow { message: MidiMessage },
}

#[derive(Debug, Default, Clone, Copy)]
pub enum MidiMessage {
    On {
        note: u8,
        velocity: u8,
        channel: u8,
    },
    Off {
        note: u8,
        velocity: u8,
        channel: u8,
    },
    Cc {
        controller: u8,
        value: u8,
        channel: u8,
    },
    #[default]
    TimingTick,
    StartSong,
    ContinueSong,
    StopSong,
}

impl Display for MidiMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            MidiMessage::On {
                note,
                velocity,
                channel,
            } => format!("note on\n\tchannel: {channel}\n\tpitch: {note}\n\tvelocity: {velocity}"),
            MidiMessage::Off {
                note,
                velocity,
                channel,
            } => format!("note off\n\tchannel: {channel}\n\tpitch: {note}\n\tvelocity: {velocity}"),
            MidiMessage::Cc {
                controller,
                value,
                channel,
            } => format!("control change\n\tchannel: {channel}\n\tcontroller: {controller}\n\tvalue: {value}"),
            MidiMessage::TimingTick => String::from("timing tick"),
            MidiMessage::StartSong => String::from("start song"),
            MidiMessage::ContinueSong => String::from("continue song"),
            MidiMessage::StopSong => String::from("stop song"),
        };

        write!(f, "{str}")
    }
}

static DATA_BIT: u8 = 0b0111_1111;
static STATUS_BIT: u8 = 0b1111_1111;

impl MidiMessage {
    pub fn as_bytes(&self) -> Result<[Option<u8>; 3], MidiError> {
        let mut bytes = [None; 3];
        match self {
            MidiMessage::On {
                note,
                velocity,
                channel,
            } => {
                if *note > 127 || *velocity > 127 {
                    return Err(MidiError::ValueOverflow { message: *self });
                } else if *channel > 15 {
                    return Err(MidiError::ChannelOverflow { message: *self });
                }
                bytes[0] = Some(STATUS_BIT & (0x90 + channel));
                bytes[1] = Some(DATA_BIT & note);
                bytes[2] = Some(DATA_BIT & velocity);
            }
            MidiMessage::Off {
                note,
                velocity,
                channel,
            } => {
                if *note > 127 || *velocity > 127 {
                    return Err(MidiError::ValueOverflow { message: *self });
                } else if *channel > 15 {
                    return Err(MidiError::ChannelOverflow { message: *self });
                }
                bytes[0] = Some(STATUS_BIT & (0x80 + channel));
                bytes[1] = Some(DATA_BIT & note);
                bytes[2] = Some(DATA_BIT & velocity);
            }
            MidiMessage::Cc {
                controller,
                value,
                channel,
            } => {
                if *controller > 127 || *value > 127 {
                    return Err(MidiError::ValueOverflow { message: *self });
                } else if *channel > 15 {
                    return Err(MidiError::ChannelOverflow { message: *self });
                }
                bytes[0] = Some(STATUS_BIT & (0xD0 + channel));
                bytes[1] = Some(DATA_BIT & controller);
                bytes[2] = Some(DATA_BIT & value);
            }
            MidiMessage::TimingTick => bytes[0] = Some(0xF8),
            MidiMessage::StartSong => bytes[0] = Some(0xFA),
            MidiMessage::ContinueSong => bytes[0] = Some(0xFB),
            MidiMessage::StopSong => bytes[0] = Some(0xFC),
        }
        Ok(bytes)
    }
}

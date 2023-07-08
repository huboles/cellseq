use std::{collections::HashSet, fmt::Display};

use eyre::Result;
use rand::random;
use thiserror::Error;
use tokio::sync::mpsc::Sender;

use crate::music::{
    generate_note, generate_velocity, Accidental, Octave, Root, RootNote, Scale, Velocity,
};

#[derive(Debug, Clone, Copy)]
pub struct MidiInfo {
    pub channel: u8,
    pub velocity: Velocity,
    pub octave: Octave,
    pub scale: Scale,
    pub root: Root,
    pub voices: u8,
    pub probability: f32,
}

impl Default for MidiInfo {
    fn default() -> Self {
        Self {
            channel: 0,
            velocity: Velocity::new(64, 127),
            octave: Octave::default(),
            scale: Scale::Chromatic,
            root: Root {
                note: RootNote::C,
                accidental: Accidental::Natural,
            },
            voices: 6,
            probability: 0.5,
        }
    }
}

#[derive(Clone, Debug)]
pub struct MidiLink {
    buffer: Vec<MidiMessage>,
    channel: Sender<u8>,
    notes_on: HashSet<u8>,
}

impl Default for MidiLink {
    fn default() -> Self {
        let (send, _) = tokio::sync::mpsc::channel(128);
        Self {
            channel: send,
            buffer: Vec::default(),
            notes_on: HashSet::default(),
        }
    }
}

impl MidiLink {
    pub fn new(channel: Sender<u8>) -> Self {
        Self {
            channel,
            ..Self::default()
        }
    }

    pub fn channel_handle(&self) -> Sender<u8> {
        self.channel.clone()
    }

    pub fn update(&mut self, hits: u8, info: &MidiInfo) {
        let mut count = 0;

        for _ in 0..hits {
            if count > info.voices {
                break;
            } else if random::<f32>() < info.probability {
                count += 1;
                continue;
            } else {
                count += 1;
                let note = generate_note(info);

                if self.notes_on.contains(&note) {
                    self.notes_on.remove(&note);
                    self.buffer.push(MidiMessage::Off {
                        note,
                        velocity: generate_velocity(info.velocity),
                        channel: info.channel,
                    });
                } else {
                    if self.notes_on.len() > info.voices.into() {
                        if let Some(elem) = self.notes_on.iter().next().cloned() {
                            self.notes_on.remove(&elem);
                            self.buffer.push(MidiMessage::Off {
                                note: elem,
                                velocity: generate_velocity(info.velocity),
                                channel: info.channel,
                            });
                        }
                    }
                    self.notes_on.insert(note);
                    self.buffer.push(MidiMessage::On {
                        note,
                        velocity: generate_velocity(info.velocity),
                        channel: info.channel,
                    });
                }
            }
        }
    }

    pub fn tick(&mut self) -> Vec<u8> {
        let vec: Vec<u8> = self
            .buffer
            .iter()
            .flat_map(|m| m.as_bytes().unwrap())
            .flatten()
            .collect();

        self.buffer.clear();
        vec
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

static DATA_MASK: u8 = 0b0111_1111;
static STATUS_MASK: u8 = 0b1111_1111;

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
                bytes[0] = Some(STATUS_MASK & (0x90 + channel));
                bytes[1] = Some(DATA_MASK & note);
                bytes[2] = Some(DATA_MASK & velocity);
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
                bytes[0] = Some(STATUS_MASK & (0x80 + channel));
                bytes[1] = Some(DATA_MASK & note);
                bytes[2] = Some(DATA_MASK & velocity);
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
                bytes[0] = Some(STATUS_MASK & (0xD0 + channel));
                bytes[1] = Some(DATA_MASK & controller);
                bytes[2] = Some(DATA_MASK & value);
            }
            MidiMessage::TimingTick => bytes[0] = Some(0xF8),
            MidiMessage::StartSong => bytes[0] = Some(0xFA),
            MidiMessage::ContinueSong => bytes[0] = Some(0xFB),
            MidiMessage::StopSong => bytes[0] = Some(0xFC),
        }
        Ok(bytes)
    }
}

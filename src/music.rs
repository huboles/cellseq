use std::fmt::Display;

use rand::random;

use crate::MidiInfo;

#[derive(Clone, Copy, Eq, PartialEq, Default, Debug)]
pub enum Scale {
    #[default]
    Chromatic,
    Major,
    Minor,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Locrian,
    MinorPentatonic,
    MajorPentatonic,
    MelodicMinor,
    HarmonicMinor,
    WholeTone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Octave {
    center: u8,
    range: u8,
}

impl Octave {
    pub fn new(center: u8, range: u8) -> Self {
        Self { center, range }
    }

    pub fn set_center(&mut self, center: u8) {
        self.center = center;
    }

    pub fn set_range(&mut self, range: u8) {
        self.range = range;
    }

    pub fn center(&self) -> u8 {
        self.center
    }

    pub fn range(&self) -> u8 {
        self.range
    }
}

impl Default for Octave {
    fn default() -> Self {
        Self {
            center: 4,
            range: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Velocity {
    min: u8,
    max: u8,
}

impl Velocity {
    pub fn new(min: u8, max: u8) -> Self {
        Self { min, max }
    }

    pub fn set_min(&mut self, min: u8) {
        self.min = min;
    }

    pub fn set_max(&mut self, max: u8) {
        self.max = max;
    }

    pub fn min(&self) -> u8 {
        self.min
    }

    pub fn max(&self) -> u8 {
        self.max
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Root {
    note: RootNote,
    accidental: Accidental,
}

impl Root {
    pub fn new(note: RootNote, accidental: Accidental) -> Self {
        Self { note, accidental }
    }

    pub fn get_note(&self) -> RootNote {
        self.note
    }

    pub fn get_accidental(&self) -> Accidental {
        self.accidental
    }
}

impl From<Root> for u8 {
    fn from(val: Root) -> Self {
        let n = match val.note {
            RootNote::A => 21,
            RootNote::B => 22,
            RootNote::C => 23,
            RootNote::D => 24,
            RootNote::E => 25,
            RootNote::F => 26,
            RootNote::G => 27,
        };

        match val.accidental {
            Accidental::Natural => n,
            Accidental::Sharp => n + 1,
            Accidental::Flat => n - 1,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RootNote {
    A,
    B,
    #[default]
    C,
    D,
    E,
    F,
    G,
}

impl Display for RootNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            RootNote::A => "A",
            RootNote::B => "B",
            RootNote::C => "C",
            RootNote::D => "D",
            RootNote::E => "E",
            RootNote::F => "F",
            RootNote::G => "G",
        };

        write!(f, "{str}")
    }
}

impl RootNote {
    pub const ALL: [RootNote; 7] = [
        RootNote::A,
        RootNote::B,
        RootNote::C,
        RootNote::D,
        RootNote::E,
        RootNote::F,
        RootNote::G,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Accidental {
    #[default]
    Natural,
    Sharp,
    Flat,
}

impl Display for Accidental {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Accidental::Natural => "",
            Accidental::Sharp => "#",
            Accidental::Flat => "b",
        };
        write!(f, "{str}")
    }
}

impl Accidental {
    pub const ALL: [Accidental; 3] = [Accidental::Natural, Accidental::Sharp, Accidental::Flat];
}

impl From<Scale> for [bool; 12] {
    fn from(val: Scale) -> Self {
        match val {
            Scale::Chromatic => [true; 12],
            Scale::Major => [
                true, false, true, false, true, true, false, true, false, true, false, true,
            ],
            Scale::Minor => [
                true, false, true, true, false, true, false, true, true, false, true, false,
            ],
            Scale::HarmonicMinor => [
                true, false, true, true, false, true, false, true, true, false, false, true,
            ],
            Scale::MelodicMinor => [
                true, false, true, true, false, true, false, true, false, true, false, true,
            ],
            Scale::Dorian => [
                true, false, true, true, false, true, false, true, false, true, true, false,
            ],
            Scale::Phrygian => [
                true, true, false, true, false, true, false, true, true, false, true, false,
            ],
            Scale::Lydian => [
                true, false, true, false, true, false, true, true, false, true, false, true,
            ],
            Scale::Mixolydian => [
                true, false, true, false, true, true, false, true, false, true, true, false,
            ],
            Scale::Locrian => [
                true, true, false, true, false, true, true, false, true, false, true, false,
            ],
            Scale::MajorPentatonic => [
                true, false, true, false, true, false, false, true, false, true, false, false,
            ],
            Scale::MinorPentatonic => [
                true, false, false, true, false, true, false, true, false, false, true, false,
            ],
            Scale::WholeTone => [
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
        }
    }
}

impl Scale {
    pub const ALL: [Scale; 13] = [
        Scale::Chromatic,
        Scale::Major,
        Scale::Minor,
        Scale::Dorian,
        Scale::Phrygian,
        Scale::Lydian,
        Scale::Mixolydian,
        Scale::Locrian,
        Scale::MinorPentatonic,
        Scale::MajorPentatonic,
        Scale::MelodicMinor,
        Scale::HarmonicMinor,
        Scale::WholeTone,
    ];
}

impl Display for Scale {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Scale::Chromatic => "chromatic",
            Scale::Major => "major",
            Scale::Minor => "minor",
            Scale::Dorian => "dorian",
            Scale::Phrygian => "phrygian",
            Scale::Lydian => "lydian",
            Scale::Mixolydian => "mixolydian",
            Scale::Locrian => "locrian",
            Scale::MinorPentatonic => "minor pentatonic",
            Scale::MajorPentatonic => "major pentatonic",
            Scale::MelodicMinor => "melodic minor",
            Scale::HarmonicMinor => "harmonic minor",
            Scale::WholeTone => "whole tone",
        };

        write!(f, "{str}")
    }
}

pub fn generate_note(info: &MidiInfo) -> u8 {
    let root: u8 = info.root.into();

    let oct_mod = random::<u8>() % info.octave.range;
    let octave = if random::<bool>() {
        info.octave.center.saturating_add(oct_mod)
    } else {
        info.octave.center.saturating_sub(oct_mod)
    };

    let scale: [bool; 12] = info.scale.into();

    let degree = loop {
        let r = random::<usize>() % 12;
        if scale[r] {
            break r.try_into().unwrap();
        } else {
            continue;
        }
    };

    octave
        .saturating_mul(12)
        .saturating_add(root)
        .saturating_add(degree)
}

pub fn generate_velocity(v: Velocity) -> u8 {
    let range = v.max - v.min;
    v.min + (random::<u8>() % range)
}

use std::fmt::display;

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

impl Into<[bool; 12]> for Scale {
    fn into(self) -> [bool; 12] {
        match self {
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
            Self::WholeTone => [
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
        }
    }
}

impl std::fmt::Display for Scale {
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

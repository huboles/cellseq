use super::*;

use super::{
    Acc::{Nat, Shp},
    Note::{A, B, C, D, E, F, G},
};

pub enum Scale {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
    MajPent,
    SusEgypt,
    BlueMajPent,
    BlueMinPen,
    MinPent,
    WholeTone,
    Chromatic,
}

impl Scale {
    pub fn to_notes(&self) -> NoteMask {
        let octave = [
            C(Nat),
            C(Shp),
            D(Nat),
            D(Shp),
            E(Nat),
            F(Nat),
            F(Shp),
            G(Nat),
            G(Shp),
            A(Nat),
            A(Shp),
            B(Nat),
        ];

        let diatonic = [1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0, 1];
        let pentatonic = [1, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 0];
        let whole_tone = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0];

        fn mask_scale(notes: &[Note; 12], mask: [u8; 12]) -> NoteMask {
            let mut output = [None; 12];
            for (i, (note, mask)) in notes.iter().zip(mask.iter()).enumerate() {
                if *mask == 1 {
                    output[i] = Some(*note)
                }
            }
            output
        }

        macro_rules! rotate {
            ($s:expr,$n:expr) => {{
                $s.rotate_left($n);
                $s
            }};
        }

        match self {
            Scale::Ionian => mask_scale(&octave, diatonic),
            Scale::Dorian => rotate!(mask_scale(&octave, diatonic), 2),
            Scale::Phrygian => rotate!(mask_scale(&octave, diatonic), 4),
            Scale::Lydian => rotate!(mask_scale(&octave, diatonic), 5),
            Scale::Mixolydian => rotate!(mask_scale(&octave, diatonic), 7),
            Scale::Aeolian => rotate!(mask_scale(&octave, diatonic), 9),
            Scale::Locrian => rotate!(mask_scale(&octave, diatonic), 11),
            Scale::MajPent => mask_scale(&octave, pentatonic),
            Scale::SusEgypt => rotate!(mask_scale(&octave, pentatonic), 2),
            Scale::BlueMinPen => rotate!(mask_scale(&octave, pentatonic), 4),
            Scale::BlueMajPent => rotate!(mask_scale(&octave, pentatonic), 7),
            Scale::MinPent => rotate!(mask_scale(&octave, pentatonic), 9),
            Scale::WholeTone => mask_scale(&octave, whole_tone),
            Scale::Chromatic => octave.map(|n| Some(n)),
        }
    }
}

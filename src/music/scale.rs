use super::*;

use super::{
    Acc::{Nat, Shp},
    Note::{A, B, C, D, E, F, G},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
    pub fn to_notes(&self) -> Vec<Note> {
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

        fn mask_scale(notes: &[Note; 12], mask: [u8; 12]) -> Vec<Note> {
            notes
                .iter()
                .zip(mask.iter())
                .filter_map(|(note, mask)| if *mask == 1 { Some(*note) } else { None })
                .collect()
        }

        macro_rules! rotate {
            ($s:expr,r,$n:expr) => {{
                $s.rotate_right($n);
                $s
            }};
            ($s:expr,l,$n:expr) => {{
                $s.rotate_left($n);
                $s
            }};
        }

        match self {
            Scale::Ionian => mask_scale(&octave, diatonic),
            Scale::Dorian => rotate!(mask_scale(&octave, diatonic), l, 2),
            Scale::Phrygian => rotate!(mask_scale(&octave, diatonic), l, 4),
            Scale::Lydian => rotate!(mask_scale(&octave, diatonic), l, 5),
            Scale::Mixolydian => rotate!(mask_scale(&octave, diatonic), r, 5),
            Scale::Aeolian => rotate!(mask_scale(&octave, diatonic), r, 3),
            Scale::Locrian => rotate!(mask_scale(&octave, diatonic), r, 1),
            Scale::MajPent => mask_scale(&octave, pentatonic),
            Scale::SusEgypt => rotate!(mask_scale(&octave, pentatonic), l, 2),
            Scale::BlueMinPen => rotate!(mask_scale(&octave, pentatonic), l, 4),
            Scale::BlueMajPent => rotate!(mask_scale(&octave, pentatonic), r, 5),
            Scale::MinPent => rotate!(mask_scale(&octave, pentatonic), r, 3),
            Scale::WholeTone => mask_scale(&octave, whole_tone),
            Scale::Chromatic => octave.to_vec(),
        }
    }
}

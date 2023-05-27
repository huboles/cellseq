pub type NoteMask = [bool; 12];

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
        let mut diatonic: NoteMask = [
            true, false, true, false, true, true, false, true, false, true, false, true,
        ];

        let mut pentatonic: NoteMask = [
            true, false, true, false, true, false, false, true, false, true, false, false,
        ];

        macro_rules! rotate {
            ($i:ident,$e:expr) => {{
                $i.rotate_left($e);
                $i
            }};
        }

        match self {
            Scale::Ionian => diatonic,
            Scale::Dorian => rotate!(diatonic, 2),
            Scale::Phrygian => rotate!(diatonic, 4),
            Scale::Lydian => rotate!(diatonic, 5),
            Scale::Mixolydian => rotate!(diatonic, 7),
            Scale::Aeolian => rotate!(diatonic, 9),
            Scale::Locrian => rotate!(diatonic, 11),
            Scale::MajPent => pentatonic,
            Scale::SusEgypt => rotate!(pentatonic, 2),
            Scale::BlueMinPen => rotate!(pentatonic, 4),
            Scale::BlueMajPent => rotate!(pentatonic, 7),
            Scale::MinPent => rotate!(pentatonic, 9),
            Scale::Chromatic => [true; 12],
            Scale::WholeTone => [
                true, false, true, false, true, false, true, false, true, false, true, false,
            ],
        }
    }
}

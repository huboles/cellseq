use super::*;

pub struct GlobalState {
    pub world: World,
    pub layout: Layout,
    pub transport: Transport,
    pub song: Song,
    pub mask: Vec<Mask>,
    pub channels: Vec<MidiChannel>,
}

impl GlobalState {
    pub fn build() -> Result<Self> {
        let layout = Layout::build()?;

        let world = World::new(layout.cells.width(), layout.cells.height() + 1);

        let channels = Vec::new();
        let mask = Vec::new();

        let transport = Transport::new(4, 4, 120);

        let song = Song::new(None, None);

        Ok(Self {
            world,
            layout,
            transport,
            song,
            mask,
            channels,
        })
    }
}

pub struct MidiChannel {
    pub num: usize,
    pub poly_num: usize,
}

pub struct Transport {
    pub running: bool,
    pub bpm: usize,
    pub sig: TimeSignature,
    pub tick: Duration,
    pub repeat: usize,
}

impl Transport {
    pub fn new(top: usize, bottom: usize, bpm: usize) -> Self {
        Self {
            sig: TimeSignature { top, bottom },
            bpm,
            running: true,
            repeat: 0,
            tick: bpm_to_ms(bpm),
        }
    }
}

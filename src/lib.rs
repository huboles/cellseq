use iced::{
    executor,
    theme::Theme,
    time,
    widget::{column, container, row},
    window, {Alignment, Application, Command, Element, Length, Point, Subscription},
};

use itertools::Itertools;
use music::Scale;
use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};

mod display;
mod map;
mod mask;
mod midi;
mod music;

use display::*;
use map::*;
use mask::*;
pub use midi::*;
use music::*;

pub type CellMap = FxHashSet<Cell>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Cell {
    i: isize,
    j: isize,
}

impl Cell {
    const SIZE: usize = 16;

    fn at(position: Point) -> Cell {
        let i = (position.y / Cell::SIZE as f32).ceil() as isize;
        let j = (position.x / Cell::SIZE as f32).ceil() as isize;

        Cell {
            i: i.saturating_sub(1),
            j: j.saturating_sub(1),
        }
    }

    fn cluster(cell: Cell) -> impl Iterator<Item = Cell> {
        let rows = cell.i.saturating_sub(1)..=cell.i.saturating_add(1);
        let columns = cell.j.saturating_sub(1)..=cell.j.saturating_add(1);

        rows.cartesian_product(columns).map(|(i, j)| Cell { i, j })
    }

    fn neighbors(cell: Cell) -> impl Iterator<Item = Cell> {
        Cell::cluster(cell).filter(move |candidate| *candidate != cell)
    }
}

#[derive(Default)]
pub struct CellSeq {
    map: Map,
    mask: Mask,
    midi: MidiLink,
    song: SongInfo,
    info: MidiInfo,
}

#[derive(Copy, Clone, Debug)]
pub struct SongInfo {
    pub is_playing: bool,
    pub bpm: usize,
    pub divisor: usize,
    pub is_looping: bool,
    pub loop_len: usize,
    pub step_num: usize,
}

impl Default for SongInfo {
    fn default() -> Self {
        Self {
            is_playing: false,
            bpm: 120,
            divisor: 4,
            is_looping: false,
            loop_len: 16,
            step_num: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    MapMessage(map::Message),
    MaskMessage(mask::Message),
    NewMap(CellMap),
    HitCount(u8),
    Tick(Instant),
    RandomizeMap,
    RandomizeMask,
    Reset,
    ClearMap,
    ClearMask,
    Save,
    TogglePlayback,
    SpeedChanged(usize),
    NewDivision(usize),
    ToggleLoop,
    LoopLength(usize),
    ProbChanged(f32),
    RandChanged(f32),
    NewVMin(u8),
    NewVMax(u8),
    ChannelChange(u8),
    Scale(Scale),
    NewOctave(u8),
    OctaveRange(u8),
    NewNote(Root),
    Voices(u8),
    Quit,
}

impl CellSeq {
    fn control_message(&self) -> ControlMessage {
        ControlMessage {
            randomness: self.map.randomness(),
            info: self.info.clone(),
            song: self.song.clone(),
        }
    }
}

impl Application for CellSeq {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = MidiLink;

    fn new(flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Self {
                midi: flags,
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("cellseq")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::None => {}
            Message::MapMessage(message) => self.map.update(message),
            Message::MaskMessage(message) => self.mask.update(message),
            Message::HitCount(x) => self.midi.update(x, &self.info),
            Message::NewMap(m) => {
                self.map.update(map::Message::Ticked(m.clone()));
                let hits = self.mask.tick(m);
                return Command::perform(async move { hits }, Message::HitCount);
            }
            Message::Tick(_) => {
                let map = if self.song.is_looping && self.song.step_num > self.song.loop_len {
                    self.song.step_num = 0;
                    self.map.reset_loop()
                } else {
                    self.song.step_num += 1;
                    self.map.tick()
                };

                let channel = self.midi.channel_handle();
                let bytes = self.midi.tick();

                let midi = tokio::spawn(async move {
                    for byte in bytes {
                        channel.send(byte).await.unwrap()
                    }
                });

                let mut commands = Vec::new();
                commands.push(Command::perform(async move { map }, Message::NewMap));
                commands.push(Command::perform(midi, |_| Message::None));
                commands.push(Command::perform(async move {}, |_| {
                    Message::MaskMessage(mask::Message::Ticked)
                }));

                return Command::batch(commands);
            }
            Message::TogglePlayback => {
                self.song.is_playing = !self.song.is_playing;
                if self.song.is_playing {
                    self.map.save()
                }
            }
            Message::ToggleLoop => {
                self.song.is_looping = !self.song.is_looping;
                if self.song.is_looping {
                    self.song.step_num = 0;
                }
            }
            Message::RandChanged(r) => {
                self.map.set_randomness(r);
                self.mask.set_randomness(r);
            }
            Message::RandomizeMap => self.map.randomize(),
            Message::RandomizeMask => self.mask.randomize(),
            Message::ClearMap => self.map.clear(),
            Message::ClearMask => self.mask.clear(),
            Message::Reset => self.map.reset(),
            Message::Save => self.map.save(),
            Message::SpeedChanged(b) => self.song.bpm = b,
            Message::NewDivision(d) => self.song.divisor = d,
            Message::LoopLength(l) => self.song.loop_len = l,
            Message::ProbChanged(p) => self.info.probability = p,
            Message::NewVMin(v) => self.info.velocity.set_min(v),
            Message::NewVMax(v) => self.info.velocity.set_max(v),
            Message::ChannelChange(c) => self.info.channel = c,
            Message::Scale(s) => self.info.scale = s,
            Message::NewOctave(o) => self.info.octave.center = o,
            Message::OctaveRange(r) => self.info.octave.range = r,
            Message::NewNote(r) => self.info.root = r,
            Message::Voices(v) => self.info.voices = v,
            Message::Quit => return window::close(),
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.song.is_playing {
            time::every(Duration::from_millis(
                240000 / (self.song.bpm * self.song.divisor) as u64,
            ))
            .map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let top = top_controls(self.song.is_playing);

        let map = row![
            self.map.view().map(Message::MapMessage),
            self.mask.view().map(Message::MaskMessage)
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .spacing(40);

        let bottom = bottom_controls(self.control_message());

        let content = column![top, map, bottom].width(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

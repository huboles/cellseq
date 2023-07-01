use iced::{
    executor,
    theme::Theme,
    time,
    widget::{column, container, row},
    {Alignment, Application, Command, Element, Length, Point, Subscription},
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
    info: MidiInfo,
    is_playing: bool,
    bpm: usize,
    is_looping: bool,
    loop_len: usize,
    step_num: usize,
    randomness: f32,
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    MapMessage(map::Message),
    MaskMessage(mask::Message),
    NewMap(CellMap),
    HitCount(u8),
    Tick(Instant),
    Randomize,
    Reset,
    Clear,
    Save,
    TogglePlayback,
    SpeedChanged(usize),
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
        todo!()
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
                bpm: 120,
                loop_len: 16,
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
            Message::NewMap(m) => {
                self.map.update(map::Message::Ticked(m.clone()));
                let hits = self.mask.tick(m);
                return Command::perform(async move { hits }, Message::HitCount);
            }
            Message::HitCount(x) => self.midi.update(x, &self.info),
            Message::MapMessage(message) => self.map.update(message),
            Message::MaskMessage(message) => self.mask.update(message),
            Message::Tick(_) => {
                let map = if self.is_looping && self.step_num > self.loop_len {
                    self.step_num = 0;
                    self.map.reset_loop()
                } else {
                    self.step_num += 1;
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

                return Command::batch(commands);
            }
            Message::TogglePlayback => {
                self.is_playing = !self.is_playing;
                if self.is_playing {
                    self.map.save()
                }
            }
            Message::SpeedChanged(bpm) => self.bpm = bpm,
            Message::Clear => self.map.clear(),
            Message::Randomize => self.map.randomize(self.randomness),
            Message::Reset => self.map.reset(),
            Message::Save => self.map.save(),
            Message::ToggleLoop => {
                self.is_looping = !self.is_looping;
                if self.is_looping {
                    self.step_num = 0;
                }
            }
            Message::LoopLength(len) => self.loop_len = len,
            Message::Quit => todo!(),
            Message::ProbChanged(p) => self.info.probability = p,
            Message::RandChanged(r) => self.randomness = r,
            Message::NewVMin(v) => self.info.velocity.set_min(v),
            Message::NewVMax(v) => self.info.velocity.set_max(v),
            Message::ChannelChange(c) => self.info.channel = c,
            Message::Scale(s) => self.info.scale = s,
            Message::NewOctave(o) => self.info.octave.set_center(o),
            Message::OctaveRange(r) => self.info.octave.set_range(r),
            Message::NewNote(r) => self.info.root = r,
            Message::Voices(v) => self.info.voices = v,
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        if self.is_playing {
            time::every(Duration::from_millis(60000 / self.bpm as u64)).map(Message::Tick)
        } else {
            Subscription::none()
        }
    }

    fn view(&self) -> Element<Message> {
        let top = top_controls(self.is_playing);

        let map = row![
            self.map.view().map(Message::MapMessage),
            self.mask.view().map(Message::MaskMessage)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .spacing(40)
        .padding(20);

        let bottom = bottom_controls(self.control_message());

        let content = column![top, map, bottom];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

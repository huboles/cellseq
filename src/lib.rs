use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Application, Command, Element, Length, Point, Subscription};

use rustc_hash::FxHashSet;
use std::time::{Duration, Instant};

mod map;
mod mask;

use map::*;
use mask::*;

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
        use itertools::Itertools;

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
    is_playing: bool,
    bpm: usize,
    is_looping: bool,
    loop_len: usize,
    step_num: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Map(map::Message),
    Mask(mask::Message),
    Tick(Instant),
    Randomize,
    Reset,
    Clear,
    Save,
    TogglePlayback,
    SpeedChanged(usize),
    ToggleLoop,
    LoopLength(usize),
}

impl Application for CellSeq {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                bpm: 120,
                loop_len: 16,
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
            Message::Map(message) => self.map.update(message),
            Message::Mask(message) => self.mask.update(message),
            Message::Tick(_) => {
                let life = if self.step_num == self.loop_len && self.is_looping {
                    self.step_num = 0;
                    self.map.reset_loop()
                } else {
                    self.step_num += 1;
                    self.map.tick()
                };

                self.map.update(map::Message::Ticked(life.clone()));
                self.mask.update(mask::Message::Tick(life));
            }
            Message::TogglePlayback => {
                self.is_playing = !self.is_playing;
                if self.is_playing {
                    self.map.save()
                }
            }
            Message::SpeedChanged(bpm) => self.bpm = bpm,
            Message::Clear => self.map.clear(),
            Message::Randomize => self.map.randomize(),
            Message::Reset => self.map.reset(),
            Message::Save => self.map.save(),
            Message::ToggleLoop => {
                self.is_looping = !self.is_looping;
                if self.is_looping {
                    self.step_num = 0;
                }
            }
            Message::LoopLength(len) => self.loop_len = len,
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
        let controls = view_controls(
            self.is_playing,
            self.bpm,
            self.is_looping,
            self.loop_len,
            self.step_num,
        );
        let map = row![
            self.map.view().map(Message::Map),
            self.mask.view().map(Message::Mask)
        ]
        .align_items(Alignment::Center)
        .width(Length::Fill)
        .spacing(40)
        .padding(20);

        let content = column![controls, map,];

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn view_controls<'a>(
    is_playing: bool,
    bpm: usize,
    is_looping: bool,
    loop_len: usize,
    step_num: usize,
) -> Element<'a, Message> {
    let playback_controls = row![
        button(if is_playing { "pause" } else { "play" }).on_press(Message::TogglePlayback),
        button(if is_looping { "free" } else { "loop" }).on_press(Message::ToggleLoop),
        button("-").on_press(Message::LoopLength(loop_len.saturating_sub(1))),
        text(if is_looping {
            format!("{step_num}/{loop_len}")
        } else {
            format!("{loop_len}")
        }),
        button("+").on_press(Message::LoopLength(loop_len.saturating_add(1)))
    ]
    .spacing(10);

    let speed_controls = row![
        button("<<").on_press(Message::SpeedChanged(bpm.saturating_sub(5))),
        button("<").on_press(Message::SpeedChanged(bpm.saturating_sub(1))),
        text(format!("{bpm}")).size(16),
        button(">").on_press(Message::SpeedChanged(bpm.saturating_add(1))),
        button(">>").on_press(Message::SpeedChanged(bpm.saturating_add(1))),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    let other_controls = row![
        button("save").on_press(Message::Save),
        button("reset")
            .on_press(Message::Reset)
            .style(theme::Button::Secondary),
        button("random")
            .on_press(Message::Randomize)
            .style(theme::Button::Positive),
        button("clear")
            .on_press(Message::Clear)
            .style(theme::Button::Destructive),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    row![playback_controls, speed_controls, other_controls]
        .padding(10)
        .spacing(40)
        .align_items(Alignment::Center)
        .into()
}

use iced::executor;
use iced::theme::{self, Theme};
use iced::time;
use iced::widget::{button, column, container, row, slider, text};
use iced::{Alignment, Application, Command, Element, Length, Point, Subscription};
use std::time::{Duration, Instant};

mod life;
mod map;
mod mask;
mod state;

use life::*;
use map::*;

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
    is_playing: bool,
    queued_ticks: usize,
    bpm: usize,
    next_speed: Option<usize>,
    version: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Grid(map::Message, usize),
    Tick(Instant),
    TogglePlayback,
    Randomize,
    Reset,
    Clear,
    Save,
    SpeedChanged(usize),
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
                ..Self::default()
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Game of Life - Iced")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Grid(message, version) => {
                if version == self.version {
                    self.map.update(message);
                }
            }
            Message::Tick(_) => {
                self.queued_ticks = (self.queued_ticks + 1).min(self.bpm);

                if let Some(task) = self.map.tick(self.queued_ticks) {
                    if let Some(speed) = self.next_speed.take() {
                        self.bpm = speed;
                    }

                    self.queued_ticks = 0;

                    let version = self.version;

                    return Command::perform(task, move |message| Message::Grid(message, version));
                }
            }
            Message::TogglePlayback => {
                self.is_playing = !self.is_playing;
            }
            Message::Clear => {
                self.map.clear();
                self.version += 1;
            }
            Message::SpeedChanged(bpm) => {
                self.bpm = bpm;
            }
            Message::Randomize => self.map.randomize(),
            Message::Reset => self.map.reset(),
            Message::Save => self.map.save(),
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
        let version = self.version;
        let selected_speed = self.next_speed.unwrap_or(self.bpm);
        let controls = view_controls(self.is_playing, selected_speed);
        let map = self.map.view().map(move |m| Message::Grid(m, version));

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

fn view_controls<'a>(is_playing: bool, bpm: usize) -> Element<'a, Message> {
    let playback_controls =
        row![button(if is_playing { "pause" } else { "play" }).on_press(Message::TogglePlayback),]
            .spacing(10);

    let speed_controls = row![
        slider(1.0..=1000.0, bpm as f32, |m| Message::SpeedChanged(
            m.round() as usize
        )),
        text(format!("{bpm}")).size(16),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    row![
        playback_controls,
        speed_controls,
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
    .padding(10)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

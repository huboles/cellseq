use iced::{
    mouse::Interaction,
    mouse::{Button::Left, Event::ButtonPressed},
    widget::canvas::{
        event::{self, Event},
        Cache, Canvas, Cursor, Geometry, Path, Program,
    },
    {Color, Element, Length, Point, Rectangle, Size, Theme},
};

use crate::{Cell, CellMap, MidiMessage};
use itertools::Itertools;
use rustc_hash::FxHashMap;

#[derive(Debug, Clone)]
pub enum Message {
    Check(Cell),
    Uncheck(Cell),
    Tick(CellMap),
}

#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum State {
    #[default]
    Off,
    On,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Note {
    value: usize,
    state: State,
}

impl Note {
    pub fn is_on(&self) -> bool {
        match self.state {
            State::On => true,
            State::Off => false,
        }
    }

    pub fn switch(&mut self) {
        self.state = match self.state {
            State::On => State::Off,
            State::Off => State::On,
        }
    }
}

#[derive(Default, Debug)]
pub struct Mask {
    cells: FxHashMap<Cell, Note>,
    mask_cache: Cache,
}

impl Mask {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Check(cell) => {
                self.cells.insert(cell, Note::default());
                self.mask_cache.clear()
            }
            Message::Uncheck(cell) => {
                self.cells.remove(&cell);
                self.mask_cache.clear();
            }
            Message::Tick(life) => {
                for cell in life.iter() {
                    if self.cells.contains_key(cell) {
                        let note = self.cells.entry(*cell).or_default();
                        note.switch()
                    }
                }

                self.mask_cache.clear();
            }
        }
    }

    pub fn is_on(&self, cell: &Cell) -> bool {
        if let Some(note) = self.cells.get(cell) {
            note.is_on()
        } else {
            false
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .height(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .into()
    }
}

impl Program<Message> for Mask {
    type State = bool;
    fn draw(
        &self,
        _state: &Self::State,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![self.mask_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x26, 0x26, 0x2E));

            frame.with_save(|frame| {
                frame.scale(Cell::SIZE as f32);

                (0..24)
                    .cartesian_product(0..24)
                    .filter(|x| self.cells.contains_key(&Cell { i: x.1, j: x.0 }))
                    .for_each(|x| {
                        let color = if self.is_on(&Cell { i: x.0, j: x.1 }) {
                            Color::from_rgb8(0xF0, 0xC0, 0xC0)
                        } else {
                            Color::WHITE
                        };

                        frame.fill_rectangle(Point::new(x.0 as f32, x.1 as f32), Size::UNIT, color);
                    })
            });
        })]
    }

    fn update(
        &self,
        _state: &mut Self::State,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Some(position) = cursor.position_in(&bounds) {
            if let Event::Mouse(ButtonPressed(Left)) = event {
                let cell = Cell::at(position);
                return (
                    event::Status::Captured,
                    if self.cells.contains_key(&cell) {
                        Some(Message::Uncheck(cell))
                    } else {
                        Some(Message::Check(cell))
                    },
                );
            }
        }

        (event::Status::Ignored, None)
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> Interaction {
        Interaction::default()
    }
}

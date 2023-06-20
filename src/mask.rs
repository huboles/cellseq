use iced::mouse::Interaction;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Cursor, Geometry, Path};
use iced::{
    mouse::{Button::Left, Event::ButtonPressed},
    widget::canvas::Program,
};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme};

use crate::{Cell, CellMap};
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
    action: State,
}

#[derive(Default, Debug)]
pub struct Mask {
    cells: FxHashMap<Cell, Note>,
    mask_cache: Cache,
}

impl Mask {
    pub fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains_key(cell)
    }

    pub fn check(&mut self, cell: Cell) {
        self.cells.insert(cell, Note::default());
    }

    pub fn uncheck(&mut self, cell: Cell) {
        let _ = self.cells.remove(&cell);
    }

    pub fn get_note(&self, cell: &Cell) -> Option<&Note> {
        self.cells.get(cell)
    }

    pub fn cells(&self) -> impl Iterator<Item = &Cell> {
        self.cells.keys()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Check(cell) => self.check(cell),
            Message::Uncheck(cell) => self.uncheck(cell),
            Message::Tick(life) => {}
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
                    .filter(|x| self.contains(&Cell { i: x.1, j: x.0 }))
                    .for_each(|x| {
                        frame.fill_rectangle(
                            Point::new(x.0 as f32, x.1 as f32),
                            Size::UNIT,
                            Color::WHITE,
                        );
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
                    if self.contains(&cell) {
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

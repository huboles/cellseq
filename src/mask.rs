use iced::{
    mouse::Interaction,
    mouse::{Button::Left, Event::ButtonPressed},
    widget::canvas::{
        event::{self, Event},
        Cache, Canvas, Cursor, Geometry, Path, Program,
    },
    {Color, Element, Length, Point, Rectangle, Size, Theme},
};
use rand::random;

use crate::{Cell, CellMap};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub enum Message {
    Check(Cell),
    Uncheck(Cell),
    Ticked,
}

#[derive(Debug)]
pub struct Mask {
    cells: FxHashSet<Cell>,
    hits: FxHashSet<Cell>,
    mask_cache: Cache,
    randomness: f32,
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            cells: FxHashSet::default(),
            mask_cache: Cache::default(),
            randomness: 0.5,
            hits: FxHashSet::default(),
        }
    }
}

impl Mask {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::Check(cell) => {
                self.cells.insert(cell);
                self.mask_cache.clear()
            }
            Message::Uncheck(cell) => {
                self.cells.remove(&cell);
                self.mask_cache.clear();
            }
            Message::Ticked => self.mask_cache.clear(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .height(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .into()
    }

    pub fn tick(&mut self, life: CellMap) -> u8 {
        self.hits.clear();
        for cell in self.cells.iter() {
            if life.contains(cell) {
                self.hits.insert(*cell);
            }
        }
        self.hits.len().try_into().unwrap_or_default()
    }

    pub fn randomize(&mut self) {
        self.cells.clear();
        for (i, j) in (-32..=32).cartesian_product(-32..=32) {
            if random::<f32>() < self.randomness {
                self.cells.insert(Cell { i, j });
            }
        }
        self.mask_cache.clear();
    }

    pub fn set_randomness(&mut self, value: f32) {
        self.randomness = value;
    }

    pub fn clear(&mut self) {
        self.cells.clear();
        self.mask_cache.clear();
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
            frame.fill(&background, Color::from_rgb8(0x10, 0x10, 0x10));

            frame.with_save(|frame| {
                frame.scale(Cell::SIZE as f32);

                (0..24)
                    .cartesian_product(0..24)
                    .filter(|x| self.cells.contains(&Cell { i: x.1, j: x.0 }))
                    .for_each(|x| {
                        frame.fill_rectangle(
                            Point::new(x.0 as f32, x.1 as f32),
                            Size::UNIT,
                            if self.hits.contains(&Cell { i: x.1, j: x.0 }) {
                                Color::from_rgb8(0xFF, 0xFF, 0xFF)
                            } else {
                                Color::from_rgb8(0xDD, 0xDD, 0xDD)
                            },
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
                    if self.cells.contains(&cell) {
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

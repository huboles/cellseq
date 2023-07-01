use iced::{
    mouse::Interaction,
    mouse::{Button::Left, Event::ButtonPressed},
    widget::canvas::{
        event::{self, Event},
        Cache, Canvas, Cursor, Geometry, Path, Program,
    },
    {Color, Element, Length, Point, Rectangle, Size, Theme},
};

use crate::{Cell, CellMap};
use itertools::Itertools;
use rustc_hash::FxHashSet;

#[derive(Debug, Clone)]
pub enum Message {
    Check(Cell),
    Uncheck(Cell),
}

#[derive(Default, Debug)]
pub struct Mask {
    cells: FxHashSet<Cell>,
    mask_cache: Cache,
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
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .height(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .into()
    }

    pub fn tick(&mut self, life: CellMap) -> u8 {
        let mut hits = 0;
        for cell in self.cells.iter() {
            if life.contains(cell) {
                hits += 1;
            }
        }
        hits
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
                    .filter(|x| self.cells.contains(&Cell { i: x.1, j: x.0 }))
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

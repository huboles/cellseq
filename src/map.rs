use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Cursor, Geometry, Path};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme};
use itertools::Itertools;
use std::future::Future;

use crate::{state::State, Cell, Life};

pub struct Map {
    state: State,
    life_cache: Cache,
}

#[derive(Debug, Clone)]
pub enum Message {
    Populate(Cell),
    Unpopulate(Cell),
    Ticked(Life),
}

impl Default for Map {
    fn default() -> Self {
        Self {
            state: State::with_life(Life::default()),
            life_cache: Cache::default(),
        }
    }
}

impl Map {
    pub fn tick(&mut self, amount: usize) -> Option<impl Future<Output = Message>> {
        let tick = self.state.tick(amount)?;

        Some(async move { Message::Ticked(tick.await) })
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Populate(cell) => {
                self.state.populate(cell);
                self.life_cache.clear();
            }
            Message::Unpopulate(cell) => {
                self.state.unpopulate(&cell);
                self.life_cache.clear();
            }
            Message::Ticked(life) => {
                self.state.update(life);
                self.life_cache.clear();
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .height(Length::Fixed(Cell::SIZE as f32 * 24.0))
            .into()
    }

    pub fn clear(&mut self) {
        self.state.clear();
        self.life_cache.clear();
    }

    pub fn reset(&mut self) {
        self.state.reset();
    }

    pub fn save(&mut self) {
        self.state.save();
    }

    pub fn randomize(&mut self) {
        self.life_cache.clear();
        self.state.randomize();
    }
}

impl canvas::Program<Message> for Map {
    type State = bool;

    fn update(
        &self,
        _interaction: &mut bool,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        if let Some(pos) = cursor.position_in(&bounds) {
            if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                let location = Point { x: pos.x, y: pos.y };

                let cell = Cell::at(location);
                return (
                    event::Status::Captured,
                    if self.state.contains(&cell) {
                        Some(Message::Unpopulate(cell))
                    } else {
                        Some(Message::Populate(cell))
                    },
                );
            }
        }

        (event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _interaction: &bool,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        vec![self.life_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x2E, 0x26, 0x2D));

            frame.with_save(|frame| {
                frame.scale(Cell::SIZE as f32);

                (0..24)
                    .cartesian_product(0..24)
                    .filter(|(i, j)| self.state.contains(&Cell { i: *i, j: *j }))
                    .for_each(|(i, j)| {
                        frame.fill_rectangle(
                            Point::new(j as f32, i as f32),
                            Size::UNIT,
                            Color::WHITE,
                        );
                    })
            });
        })]
    }

    fn mouse_interaction(
        &self,
        _interaction: &bool,
        _bounds: Rectangle,
        _cursor: Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}

use iced::alignment;
use iced::mouse;
use iced::widget::canvas;
use iced::widget::canvas::event::{self, Event};
use iced::widget::canvas::{Cache, Canvas, Cursor, Frame, Geometry, Path, Text};
use iced::{Color, Element, Length, Point, Rectangle, Size, Theme, Vector};
use std::future::Future;
use std::ops::RangeInclusive;
use std::time::{Duration, Instant};

use crate::{mask::Note, state::State, Cell, Life};

pub struct Map {
    state: State,
    life_cache: Cache,
    mask_cache: Cache,
    translation: Vector,
    last_tick_duration: Duration,
    last_queued_ticks: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    Populate(Cell),
    Unpopulate(Cell),
    Check(Cell),
    Uncheck(Cell),
    Ticked {
        result: Result<Life, TickError>,
        tick_duration: Duration,
    },
}

#[derive(Debug, Clone)]
pub enum TickError {
    JoinFailed,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            state: State::with_life(Life::default()),
            life_cache: Cache::default(),
            mask_cache: Cache::default(),
            translation: Vector::default(),
            last_tick_duration: Duration::default(),
            last_queued_ticks: 0,
        }
    }
}

impl Map {
    pub fn tick(&mut self, amount: usize) -> Option<impl Future<Output = Message>> {
        let tick = self.state.tick(amount)?;

        self.last_queued_ticks = amount;

        Some(async move {
            let start = Instant::now();
            let result = tick.await;
            let tick_duration = start.elapsed() / amount as u32;

            Message::Ticked {
                result,
                tick_duration,
            }
        })
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Check(cell) => {
                self.state.check(cell);
                self.mask_cache.clear();
            }
            Message::Uncheck(cell) => {
                self.state.uncheck(cell);
                self.mask_cache.clear();
            }
            Message::Populate(cell) => {
                self.state.populate(cell);
                self.life_cache.clear();
            }
            Message::Unpopulate(cell) => {
                self.state.unpopulate(&cell);
                self.life_cache.clear();
            }
            Message::Ticked {
                result: Ok(life),
                tick_duration,
            } => {
                self.state.update(life);
                self.life_cache.clear();

                self.last_tick_duration = tick_duration;
            }
            Message::Ticked {
                result: Err(error), ..
            } => {
                dbg!(error);
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn clear(&mut self) {
        self.state = State::default();
        self.life_cache.clear();
    }

    fn visible_region(&self, size: Size) -> Region {
        let width = size.width;
        let height = size.height;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width,
            height,
        }
    }

    fn project(&self, position: Point, size: Size, in_life: bool) -> Point {
        let region = self.visible_region(size);

        let center = Point {
            x: size.width / 2.0,
            y: size.height / 2.0,
        };

        let translation = Vector {
            x: 0.0,
            y: if in_life {
                center.y - center.y / 2.0
            } else {
                center.y + center.y / 2.0
            },
        };

        Point::new(position.x + region.x, position.y + region.y) + translation
    }
}

impl canvas::Program<Message> for Map {
    type State = Interaction;

    fn update(
        &self,
        interaction: &mut Interaction,
        event: Event,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> (event::Status, Option<Message>) {
        let center = bounds.center();

        if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
            *interaction = Interaction::None;
        }

        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position
        } else {
            return (event::Status::Ignored, None);
        };

        let cell: Cell;
        let action: Option<Message>;
        let is_populated: bool;

        if cursor_position.y < center.y {
            cell = Cell::at(self.project(cursor_position, bounds.size(), true));
            is_populated = self.state.contains(&cell);

            action = if is_populated {
                Some(Message::Unpopulate(cell))
            } else {
                Some(Message::Populate(cell))
            };
        } else {
            cell = Cell::at(self.project(cursor_position, bounds.size(), false));
            is_populated = self.state.mask_contains(&cell);

            action = if is_populated {
                Some(Message::Uncheck(cell))
            } else {
                Some(Message::Check(cell))
            };
        }

        match event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                *interaction = if is_populated {
                    Interaction::Erasing
                } else {
                    Interaction::Drawing
                };
                (event::Status::Captured, action)
            }
            _ => (event::Status::Ignored, None),
        }
    }

    fn draw(
        &self,
        _interaction: &Interaction,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: Cursor,
    ) -> Vec<Geometry> {
        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let split_half = Size::new(bounds.width, bounds.height / 2.0);

        let life_canvas = Rectangle::new(bounds.position(), split_half);
        let mask_canvas =
            Rectangle::new(bounds.position() + Vector::new(0.0, center.y), split_half);

        let life = self.life_cache.draw(life_canvas.size(), |frame| {
            let life_center = Vector {
                x: bounds.x + life_canvas.center().x,
                y: bounds.y + life_canvas.center().y,
            };
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0xA0, 0x44, 0x4B));

            frame.with_save(|frame| {
                frame.translate(life_center);
                frame.scale(Cell::SIZE as f32);

                let region = self.visible_region(frame.size());

                for cell in region.cull(self.state.cells()) {
                    frame.fill_rectangle(
                        Point::new(cell.j as f32, cell.i as f32),
                        Size::UNIT,
                        Color::WHITE,
                    );
                }
            });
        });

        let mask = self.mask_cache.draw(mask_canvas.size(), |frame| {
            let mask_center = Vector {
                x: bounds.x + life_canvas.center().x,
                y: bounds.y + life_canvas.center().y,
            };
            let background = Path::rectangle(Point::ORIGIN, frame.size());
            frame.fill(&background, Color::from_rgb8(0x40, 0x44, 0xA0));

            frame.with_save(|frame| {
                frame.translate(mask_center);
                frame.scale(Cell::SIZE as f32);

                let region = self.visible_region(frame.size());

                for (cell, _) in region.cull_mask(self.state.mask()) {
                    frame.fill_rectangle(
                        Point::new(cell.j as f32, cell.i as f32),
                        Size::UNIT,
                        Color::WHITE,
                    );
                }
            });
        });

        let overlay = {
            let mut frame = Frame::new(bounds.size());

            let text = Text {
                color: Color::WHITE,
                size: 14.0,
                position: Point::new(frame.width(), frame.height()),
                horizontal_alignment: alignment::Horizontal::Right,
                vertical_alignment: alignment::Vertical::Bottom,
                ..Text::default()
            };

            let cell_count = self.state.cell_count();

            frame.fill_text(Text {
                content: format!(
                    "{} cell{} @ {:?} ({})",
                    cell_count,
                    if cell_count == 1 { "" } else { "s" },
                    self.last_tick_duration,
                    self.last_queued_ticks
                ),
                ..text
            });

            frame.into_geometry()
        };

        vec![overlay, mask, life]
    }

    fn mouse_interaction(
        &self,
        interaction: &Interaction,
        bounds: Rectangle,
        cursor: Cursor,
    ) -> mouse::Interaction {
        match interaction {
            Interaction::Drawing => mouse::Interaction::Crosshair,
            Interaction::Erasing => mouse::Interaction::Crosshair,
            Interaction::None if cursor.is_over(&bounds) => mouse::Interaction::Crosshair,
            _ => mouse::Interaction::default(),
        }
    }
}
pub struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

impl Region {
    fn rows(&self) -> RangeInclusive<isize> {
        let first_row = (self.y / Cell::SIZE as f32).floor() as isize;

        let visible_rows = (self.height / Cell::SIZE as f32).ceil() as isize;

        first_row..=first_row + visible_rows
    }

    fn columns(&self) -> RangeInclusive<isize> {
        let first_column = (self.x / Cell::SIZE as f32).floor() as isize;

        let visible_columns = (self.width / Cell::SIZE as f32).ceil() as isize;

        first_column..=first_column + visible_columns
    }

    fn cull<'a>(&self, cells: impl Iterator<Item = &'a Cell>) -> impl Iterator<Item = &'a Cell> {
        let rows = self.rows();
        let columns = self.columns();

        cells.filter(move |cell| rows.contains(&cell.i) && columns.contains(&cell.j))
    }

    fn cull_mask<'a>(
        &self,
        cells: impl Iterator<Item = (&'a Cell, &'a Note)>,
    ) -> impl Iterator<Item = (&'a Cell, &'a Note)> {
        let rows = self.rows();
        let columns = self.columns();

        cells.filter(move |(cell, _)| rows.contains(&cell.i) && columns.contains(&cell.j))
    }
}

pub enum Interaction {
    None,
    Drawing,
    Erasing,
}

impl Default for Interaction {
    fn default() -> Self {
        Self::None
    }
}

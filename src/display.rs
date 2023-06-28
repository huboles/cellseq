use iced::{
    theme,
    widget::{button, column, row, slider, text},
    Alignment, Element, Length,
};

use crate::Message;

pub fn top_controls<'a>(
    is_playing: bool,
    bpm: usize,
    is_looping: bool,
    loop_len: usize,
    step_num: usize,
) -> Element<'a, Message> {
    let play_button =
        button(if is_playing { "pause" } else { "play" }).on_press(Message::TogglePlayback);

    let loop_controls = row![
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
        button(">>").on_press(Message::SpeedChanged(bpm.saturating_add(5))),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    let other_controls = row![button("quit")
        .on_press(Message::Quit)
        .style(theme::Button::Destructive),]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10);

    row![play_button, loop_controls, speed_controls, other_controls]
        .padding(10)
        .spacing(40)
        .align_items(Alignment::Center)
        .into()
}

pub fn bottom_controls<'a>(
    prob: f32,
    rand: f32,
    vmin: u8,
    vmax: u8,
    chan: u8,
) -> Element<'a, Message> {
    let probability = row![
        slider(0.0..=100.0, prob * 100.0, |x| Message::ProbChanged(
            x / 100.0
        )),
        text(format!("{prob}")),
        text("probability a cell gets triggered")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center);

    let randomize = row![
        slider(0.0..=100.0, rand * 100.0, |x| Message::RandChanged(
            x / 100.0
        )),
        text(format!("{rand}")),
        text("percent of board to fill on randomize")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center);

    let map_controls = row![
        button("save")
            .on_press(Message::Save)
            .style(theme::Button::Positive),
        button("clear")
            .on_press(Message::Clear)
            .style(theme::Button::Destructive),
        button("reset")
            .on_press(Message::Reset)
            .style(theme::Button::Secondary),
        button("random")
            .on_press(Message::Randomize)
            .style(theme::Button::Primary),
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center);

    let map = column![map_controls, randomize];

    let velocity = column![
        row![
            slider(0..=127, vmin, Message::NewVMin),
            text(format!("{vmin}")),
            text("minimum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center),
        row![
            slider(0..=127, vmax, Message::NewVMax),
            text(format!("{vmax}")),
            text("maximum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center);

    let midi = row![
        button("-").on_press(Message::ChannelChange(chan.saturating_sub(1))),
        text(format!("{chan}")),
        button("+").on_press(Message::ChannelChange(chan.saturating_add(1))),
        velocity
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center);

    column![probability, map, midi]
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(10)
        .spacing(40)
        .align_items(Alignment::Center)
        .into()
}

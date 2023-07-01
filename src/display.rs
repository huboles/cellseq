use iced::{
    theme,
    widget::{button, column, pick_list, row, slider, text, Column, Row},
    Alignment, Element, Length,
};

use crate::{
    music::{Accidental, Root, RootNote, Scale},
    Message,
};

pub struct ControlMessage {
    probability: f32,
    randomness: f32,
    velocity_min: u8,
    velocity_max: u8,
    channel: u8,
    bpm: usize,
    is_looping: bool,
    loop_len: usize,
    step_num: usize,
    octave: u8,
    range: u8,
    scale: Scale,
    root: Root,
    voices: u8,
}

pub fn top_controls<'a>(is_playing: bool) -> Element<'a, Message> {
    let play_button =
        button(if is_playing { "pause" } else { "play" }).on_press(Message::TogglePlayback);

    let other_controls = row![button("quit")
        .on_press(Message::Quit)
        .style(theme::Button::Destructive),]
    .width(Length::Fill)
    .align_items(Alignment::End)
    .spacing(10);

    row![play_button, other_controls]
        .padding(10)
        .spacing(40)
        .into()
}

pub fn bottom_controls<'a>(message: ControlMessage) -> Element<'a, Message> {
    column![
        map_section(&message),
        midi_section(&message),
        song_section(&message)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .spacing(40)
    .align_items(Alignment::Center)
    .into()
}

fn map_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        map_buttons(),
        column![
            probability_section(message.probability),
            randomize_section(message.randomness)
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn probability_section<'a>(p: f32) -> Row<'a, Message> {
    row![
        slider(0.0..=100.0, p * 100.0, |x| {
            Message::ProbChanged(x / 100.0)
        }),
        text(format!("{p}")),
        text("probability a cell gets triggered")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn randomize_section<'a>(r: f32) -> Row<'a, Message> {
    row![
        slider(0.0..=100.0, r * 100.0, |x| {
            Message::RandChanged(x / 100.0)
        }),
        text(format!("{r}")),
        text("percent of board to fill on randomize")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn map_buttons<'a>() -> Row<'a, Message> {
    row![
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
    .align_items(Alignment::Center)
}

fn velocity_sliders<'a>(min: u8, max: u8) -> Column<'a, Message> {
    column![
        row![
            slider(0..=127, min, Message::NewVMin),
            text(format!("{min}")),
            text("minimum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center),
        row![
            slider(0..=127, max, Message::NewVMax),
            text(format!("{max}")),
            text("maximum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x40, 0x40, 0x40)))
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn midi_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        channel_selector(message.channel),
        velocity_sliders(message.velocity_min, message.velocity_max)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn channel_selector<'a>(channel: u8) -> Row<'a, Message> {
    row![
        button("-").on_press(Message::ChannelChange(channel.saturating_sub(1))),
        text(format!("channel: {channel}")),
        button("+").on_press(Message::ChannelChange(channel.saturating_add(1))),
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn song_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        column![
            loop_controls(message.is_looping, message.loop_len, message.step_num),
            speed_controls(message.bpm)
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center),
        column![
            voice_controls(message.voices),
            octave_selector(message.octave, message.range),
            scale_selector(
                message.scale,
                message.root.get_note(),
                message.root.get_accidental()
            )
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn octave_selector<'a>(oct: u8, range: u8) -> Row<'a, Message> {
    row![
        button("-")
            .on_press(Message::NewOctave(oct.saturating_sub(1)))
            .style(theme::Button::Destructive),
        text(format!("octave: {oct}")),
        button("+")
            .on_press(Message::NewOctave(oct.saturating_add(1)))
            .style(theme::Button::Positive),
        button("-")
            .on_press(Message::OctaveRange(range.saturating_sub(1)))
            .style(theme::Button::Destructive),
        text(format!("range: +/-{range}")),
        button("+")
            .on_press(Message::OctaveRange(range.saturating_add(1)))
            .style(theme::Button::Positive),
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn loop_controls<'a>(looping: bool, len: usize, step: usize) -> Row<'a, Message> {
    row![
        button(if looping { "free" } else { "loop" }).on_press(Message::ToggleLoop),
        button("-").on_press(Message::LoopLength(len.saturating_sub(1))),
        text(if looping {
            format!("{step}/{len}")
        } else {
            format!("{len}")
        }),
        button("+").on_press(Message::LoopLength(len.saturating_add(1)))
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn speed_controls<'a>(bpm: usize) -> Row<'a, Message> {
    row![
        button("<<").on_press(Message::SpeedChanged(bpm.saturating_sub(5))),
        button("<").on_press(Message::SpeedChanged(bpm.saturating_sub(1))),
        text(format!("{bpm}")).size(16),
        button(">").on_press(Message::SpeedChanged(bpm.saturating_add(1))),
        button(">>").on_press(Message::SpeedChanged(bpm.saturating_add(5))),
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10)
}

fn voice_controls<'a>(voices: u8) -> Row<'a, Message> {
    row![
        button("-")
            .on_press(Message::Voices(voices.saturating_sub(1)))
            .style(theme::Button::Destructive),
        text(format!("voices: {voices}")),
        button("+")
            .on_press(Message::Voices(voices.saturating_add(1)))
            .style(theme::Button::Positive),
    ]
    .padding(10)
    .spacing(10)
    .align_items(Alignment::Center)
}

fn scale_selector<'a>(scale: Scale, note: RootNote, acc: Accidental) -> Row<'a, Message> {
    row![
        pick_list(&Scale::ALL[..], Some(scale), Message::Scale),
        pick_list(&RootNote::ALL[..], Some(note), move |note| {
            Message::NewNote(Root::new(note, acc))
        }),
        pick_list(&Accidental::ALL[..], Some(acc), move |acc| {
            Message::NewNote(Root::new(note, acc))
        })
    ]
    .width(Length::Fill)
    .align_items(Alignment::Center)
    .spacing(10)
}

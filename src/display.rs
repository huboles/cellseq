use iced::{
    theme,
    widget::{button, column, pick_list, row, slider, text, Column, Row},
    Alignment, Element, Length,
};

use crate::{
    music::{Accidental, Root, RootNote, Scale},
    Message, MidiInfo, SongInfo,
};

#[derive(Default, Copy, Clone, Debug)]
pub struct ControlMessage {
    pub randomness: f32,
    pub info: MidiInfo,
    pub song: SongInfo,
}

pub fn top_controls<'a>(is_playing: bool) -> Element<'a, Message> {
    let play_button = row![
        button(if is_playing { "stop" } else { "play" }).on_press(Message::TogglePlayback),
        button("save")
            .on_press(Message::Save)
            .style(theme::Button::Positive),
        button("clear")
            .on_press(Message::Clear)
            .style(theme::Button::Destructive),
    ]
    .width(Length::Fill)
    .spacing(10);

    let other_controls = row![button("quit")
        .on_press(Message::Quit)
        .style(theme::Button::Destructive),]
    .width(Length::Fill)
    .spacing(10);

    row![play_button, other_controls]
        .width(Length::Fill)
        .padding(10)
        .spacing(40)
        .into()
}

pub fn bottom_controls<'a>(message: ControlMessage) -> Element<'a, Message> {
    column![
        map_section(&message),
        probability_section(message.info.probability).width(Length::Fixed(600.0)),
        midi_section(&message),
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

fn map_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        map_buttons(),
        randomize_section(message.randomness).width(Length::Fixed(500.0))
    ]
    .spacing(10)
}

fn probability_section<'a>(p: f32) -> Row<'a, Message> {
    row![
        text("probability a cell gets triggered")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x60, 0x60, 0x60))),
        slider(0.0..=100.0, p * 100.0, |x| {
            Message::ProbChanged(x / 100.0)
        }),
        text(format!("{p}")),
    ]
    .spacing(10)
}

fn randomize_section<'a>(r: f32) -> Row<'a, Message> {
    row![
        text("percent of board to fill on randomize")
            .style(theme::Text::Color(iced::Color::from_rgb8(0x60, 0x60, 0x60))),
        slider(0.0..=100.0, r * 100.0, |x| {
            Message::RandChanged(x / 100.0)
        }),
        text(format!("{r}")),
    ]
    .spacing(10)
}

fn map_buttons<'a>() -> Column<'a, Message> {
    column![row![
        button("reset")
            .on_press(Message::Reset)
            .style(theme::Button::Secondary),
        button("random")
            .on_press(Message::Randomize)
            .style(theme::Button::Primary),
    ]
    .spacing(10)]
    .spacing(10)
}

fn velocity_sliders<'a>(min: u8, max: u8) -> Column<'a, Message> {
    column![
        row![
            text("maximum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x60, 0x60, 0x60))),
            slider(0..=127, max, Message::NewVMax),
            text(format!("{max}")),
        ]
        .spacing(10),
        row![
            text("minimum velocity")
                .style(theme::Text::Color(iced::Color::from_rgb8(0x60, 0x60, 0x60))),
            slider(0..=127, min, Message::NewVMin),
            text(format!("{min}")),
        ]
        .spacing(10),
    ]
    .spacing(10)
}

fn midi_section<'a>(message: &ControlMessage) -> Column<'a, Message> {
    column![
        song_section(message),
        velocity_sliders(message.info.velocity.min(), message.info.velocity.max())
            .width(Length::Fixed(500.0))
    ]
    .spacing(10)
}

fn song_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        column![
            loop_controls(
                message.song.is_looping,
                message.song.loop_len,
                message.song.step_num
            ),
            speed_controls(message.song.bpm),
            division_controls(message.song.divisor),
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center),
        column![
            voice_controls(message.info.voices, message.info.channel),
            octave_selector(message.info.octave.center(), message.info.octave.range()),
            scale_selector(
                message.info.scale,
                message.info.root.get_note(),
                message.info.root.get_accidental()
            )
        ]
        .padding(10)
        .spacing(10)
        .align_items(Alignment::Center)
    ]
    .spacing(10)
}

fn octave_selector<'a>(oct: u8, range: u8) -> Row<'a, Message> {
    row![
        button("-").on_press(Message::NewOctave(oct.saturating_sub(1))),
        text(format!("octave: {oct}")),
        button("+").on_press(Message::NewOctave(oct.saturating_add(1))),
        button("-").on_press(Message::OctaveRange(range.saturating_sub(1))),
        text(format!("range: +/-{range}")),
        button("+").on_press(Message::OctaveRange(range.saturating_add(1)))
    ]
    .spacing(10)
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
    .spacing(10)
}

fn speed_controls<'a>(bpm: usize) -> Row<'a, Message> {
    row![
        button("<<").on_press(Message::SpeedChanged(bpm.saturating_sub(5))),
        button("<").on_press(Message::SpeedChanged(bpm.saturating_sub(1))),
        text(format!("{bpm}")).size(16),
        button(">").on_press(Message::SpeedChanged(bpm.saturating_add(1))),
        button(">>").on_press(Message::SpeedChanged(bpm.saturating_add(5))),
    ]
    .spacing(10)
}

fn division_controls<'a>(divisor: usize) -> Row<'a, Message> {
    row![
        button("-").on_press(if divisor > 1 {
            Message::NewDivision(divisor.saturating_sub(1))
        } else {
            Message::None
        }),
        text(format!("note division: {divisor}")),
        button("+").on_press(Message::NewDivision(divisor.saturating_add(1)))
    ]
    .spacing(10)
}

fn voice_controls<'a>(voices: u8, channel: u8) -> Row<'a, Message> {
    row![
        button("-").on_press(Message::ChannelChange(channel.saturating_sub(1))),
        text(format!("channel: {channel}")),
        button("+").on_press(Message::ChannelChange(channel.saturating_add(1))),
        button("-").on_press(Message::Voices(voices.saturating_sub(1))),
        text(format!("voices: {voices}")),
        button("+").on_press(Message::Voices(voices.saturating_add(1))),
    ]
    .spacing(10)
}

fn scale_selector<'a>(scale: Scale, note: RootNote, acc: Accidental) -> Row<'a, Message> {
    row![
        pick_list(&RootNote::ALL[..], Some(note), move |note| {
            Message::NewNote(Root::new(note, acc))
        })
        .width(Length::Fixed(50.0)),
        pick_list(&Accidental::ALL[..], Some(acc), move |acc| {
            Message::NewNote(Root::new(note, acc))
        })
        .width(Length::Fixed(90.0)),
        pick_list(&Scale::ALL[..], Some(scale), Message::Scale).width(Length::Fixed(160.0)),
    ]
    .align_items(Alignment::Center)
}

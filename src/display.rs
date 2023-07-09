use iced::{
    alignment::{Horizontal, Vertical},
    theme,
    widget::{
        button, checkbox, column, container, horizontal_space, pick_list, row, slider, text,
        vertical_slider, vertical_space,
    },
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
    let play_button = container(
        row![
            button(if is_playing { "stop" } else { "play" }).on_press(Message::TogglePlayback),
            button("save map")
                .on_press(Message::Save)
                .style(theme::Button::Positive),
            button("reset map")
                .on_press(Message::Reset)
                .style(theme::Button::Secondary),
            button("clear map")
                .on_press(Message::ClearMap)
                .style(theme::Button::Destructive),
            button("clear mask")
                .on_press(Message::ClearMask)
                .style(theme::Button::Destructive),
        ]
        .spacing(10),
    )
    .align_x(Horizontal::Left);

    let other_controls = container(
        button("quit")
            .on_press(Message::Quit)
            .style(theme::Button::Destructive),
    )
    .align_x(Horizontal::Right);

    container(row![play_button, horizontal_space(Length::Fill), other_controls].width(Length::Fill))
        .padding(10)
        .align_y(Vertical::Top)
        .into()
}

pub fn bottom_controls<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        column![
            randomize_section(message.randomness),
            vertical_space(40),
            music_controls(message)
        ]
        .align_items(Alignment::Center)
        .padding(10)
        .spacing(20),
    )
    .align_x(Horizontal::Center)
    .into()
}

fn music_controls<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        row![
            column![
                scale_selector(message),
                row![song_section(message), midi_section(message),]
                    .padding(10)
                    .spacing(40),
            ]
            .align_items(Alignment::Center),
            velocity_sliders(message)
        ]
        .padding(10)
        .spacing(40),
    )
    .into()
}

fn song_section<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        row![song_params(), song_vals(message)]
            .padding(10)
            .spacing(10),
    )
    .into()
}

fn midi_section<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        row![midi_params(), midi_vals(message)]
            .padding(10)
            .spacing(10),
    )
    .into()
}

fn song_params<'a>() -> Element<'a, Message> {
    container(
        column![
            text("bpm"),
            text("note division"),
            text("number of steps"),
            text("loop section"),
        ]
        .align_items(Alignment::End)
        .padding(10)
        .spacing(30),
    )
    .into()
}

fn song_vals<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        column![
            row![
                button("-").on_press(Message::SpeedChanged(message.song.bpm.saturating_sub(1))),
                text(format!("{}", message.song.bpm)),
                button("+").on_press(Message::SpeedChanged(message.song.bpm.saturating_add(1))),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
            row![
                button("-").on_press(Message::NewDivision(message.song.divisor.saturating_sub(1))),
                text(format!("{}", message.song.divisor)),
                button("+").on_press(Message::NewDivision(message.song.divisor.saturating_add(1))),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
            row![
                button("-").on_press(Message::LoopLength(message.song.loop_len.saturating_sub(1))),
                text(if message.song.is_looping {
                    format!("{}/{}", message.song.step_num, message.song.loop_len)
                } else {
                    format!("{}", message.song.loop_len)
                }),
                button("+").on_press(Message::LoopLength(message.song.loop_len.saturating_add(1))),
            ]
            .align_items(Alignment::Center)
            .spacing(10),
            checkbox("", message.song.is_looping, |_| { Message::ToggleLoop }),
        ]
        .align_items(Alignment::Center)
        .padding(10)
        .spacing(20),
    )
    .into()
}

fn midi_params<'a>() -> Element<'a, Message> {
    container(
        column![
            text("center octave"),
            text("octave range"),
            text("number of voices"),
            text("midi channel"),
        ]
        .align_items(Alignment::End)
        .padding(10)
        .spacing(30),
    )
    .into()
}

fn midi_vals<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        column![
            row![
                button("-").on_press(Message::NewOctave(
                    message.info.octave.center.saturating_sub(1)
                )),
                text(format!("{}", message.info.octave.center)),
                button("+").on_press(Message::NewOctave(
                    message.info.octave.center.saturating_add(1)
                )),
            ]
            .spacing(10),
            row![
                button("-").on_press(Message::OctaveRange(
                    message.info.octave.range.saturating_sub(1)
                )),
                text(format!("{}", message.info.octave.range)),
                button("+").on_press(Message::OctaveRange(
                    message.info.octave.range.saturating_add(1)
                )),
            ]
            .spacing(10),
            row![
                button("-").on_press(Message::Voices(message.info.voices.saturating_sub(1))),
                text(format!("{}", message.info.voices)),
                button("+").on_press(Message::Voices(message.info.voices.saturating_add(1))),
            ]
            .spacing(10),
            row![
                button("-").on_press(Message::ChannelChange(
                    message.info.channel.saturating_sub(1)
                )),
                text(format!("{}", message.info.channel + 1)),
                button("+").on_press(Message::ChannelChange(
                    message.info.channel.saturating_add(1)
                )),
            ]
            .spacing(10)
        ]
        .align_items(Alignment::Center)
        .padding(10)
        .spacing(20),
    )
    .into()
}

fn randomize_section<'a>(r: f32) -> Element<'a, Message> {
    container(
        row![
            button("randomize map")
                .on_press(Message::RandomizeMap)
                .style(theme::Button::Primary),
            column![
                slider(0.0..=100.0, r * 100.0, |x| {
                    Message::RandChanged(x / 100.0)
                })
                .width(Length::Fixed(300.0)),
                text(format!("{r}")),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
            button("randomize mask")
                .on_press(Message::RandomizeMask)
                .style(theme::Button::Primary),
        ]
        .spacing(20),
    )
    .into()
}

fn velocity_sliders<'a>(message: ControlMessage) -> Element<'a, Message> {
    container(
        column![
            text("velocity range"),
            row![
                column![
                    text(format!("{}", message.info.velocity.max())),
                    vertical_slider(0..=127, message.info.velocity.max(), Message::NewVMax),
                    text("max")
                ],
                column![
                    text(format!("{}", message.info.velocity.min())),
                    vertical_slider(0..=127, message.info.velocity.min(), Message::NewVMin),
                    text("min")
                ],
            ]
            .spacing(20.0)
        ]
        .height(Length::Fixed(300.0))
        .spacing(10),
    )
    .into()
}

fn scale_selector<'a>(message: ControlMessage) -> Element<'a, Message> {
    let scale = message.info.scale;
    let note = message.info.root.note;
    let accidental = message.info.root.accidental;
    container(
        row![
            pick_list(&RootNote::ALL[..], Some(note), move |note| {
                Message::NewNote(Root { note, accidental })
            })
            .width(Length::Fixed(50.0)),
            pick_list(&Accidental::ALL[..], Some(accidental), move |accidental| {
                Message::NewNote(Root { note, accidental })
            })
            .width(Length::Fixed(90.0)),
            pick_list(&Scale::ALL[..], Some(scale), Message::Scale).width(Length::Fixed(160.0)),
        ]
        .spacing(10),
    )
    .align_x(Horizontal::Center)
    .into()
}

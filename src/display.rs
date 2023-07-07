use iced::{
    theme,
    widget::{
        button, checkbox, column, pick_list, row, slider, text, vertical_slider, Column, Row,
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
    let play_button = row![
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
    .width(Length::Fill)
    .spacing(10);

    let other_controls = row![button("quit")
        .on_press(Message::Quit)
        .style(theme::Button::Destructive),]
    .width(Length::Fill)
    .align_items(Alignment::End)
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
        probability_section(&message).width(Length::Fixed(600.0)),
        scale_selector(&message),
        music_controls(&message)
    ]
    .width(Length::Fill)
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .align_items(Alignment::Center)
    .into()
}

fn music_controls<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        song_section(message),
        midi_section(message),
        velocity_sliders(message)
    ]
    .height(Length::Fill)
    .padding(10)
    .spacing(40)
    .into()
}

fn song_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![song_params(), song_vals(message)]
        .height(Length::Fill)
        .padding(10)
        .spacing(20)
        .into()
}

fn midi_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![midi_params(), midi_vals(message)]
        .height(Length::Fill)
        .padding(10)
        .spacing(20)
        .into()
}

fn song_params<'a>() -> Column<'a, Message> {
    column![
        text("loop section"),
        text("number of steps"),
        text("bpm"),
        text("note division"),
    ]
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .into()
}

fn song_vals<'a>(message: &ControlMessage) -> Column<'a, Message> {
    column![
        checkbox("", message.song.is_looping, |_| { Message::ToggleLoop }),
        row![
            button("-").on_press(Message::LoopLength(message.song.loop_len.saturating_sub(1))),
            text(if message.song.is_looping {
                format!("{}/{}", message.song.step_num, message.song.loop_len)
            } else {
                format!("{}", message.song.loop_len)
            }),
            button("+").on_press(Message::LoopLength(message.song.loop_len.saturating_add(1))),
        ],
        row![
            button("-").on_press(Message::SpeedChanged(message.song.bpm.saturating_sub(1))),
            text(format!("{}", message.song.bpm)),
            button("+").on_press(Message::SpeedChanged(message.song.bpm.saturating_add(1))),
        ],
        row![
            button("-").on_press(Message::NewDivision(message.song.divisor.saturating_sub(1))),
            text(format!("{}", message.song.divisor)),
            button("+").on_press(Message::NewDivision(message.song.divisor.saturating_add(1))),
        ]
    ]
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .into()
}

fn midi_params<'a>() -> Column<'a, Message> {
    column![
        text("center octave"),
        text("octave range"),
        text("number of voices"),
        text("midi channel"),
    ]
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .into()
}

fn midi_vals<'a>(message: &ControlMessage) -> Column<'a, Message> {
    column![
        row![
            button("-").on_press(Message::NewOctave(
                message.info.octave.center.saturating_sub(1)
            )),
            text(format!("{}", message.info.octave.center)),
            button("+").on_press(Message::NewOctave(
                message.info.octave.center.saturating_add(1)
            )),
        ],
        row![
            button("-").on_press(Message::OctaveRange(
                message.info.octave.range.saturating_sub(1)
            )),
            text(format!("{}", message.info.octave.range)),
            button("+").on_press(Message::OctaveRange(
                message.info.octave.range.saturating_add(1)
            )),
        ],
        row![
            button("-").on_press(Message::Voices(message.info.voices.saturating_sub(1))),
            text(format!("{}", message.info.voices)),
            button("+").on_press(Message::Voices(message.info.voices.saturating_add(1))),
        ],
        row![
            button("-").on_press(Message::ChannelChange(
                message.info.channel.saturating_sub(1)
            )),
            text(format!("{}", message.info.channel)),
            button("+").on_press(Message::ChannelChange(
                message.info.channel.saturating_add(1)
            )),
        ]
    ]
    .height(Length::Fill)
    .padding(10)
    .spacing(20)
    .into()
}

fn map_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        map_buttons(),
        randomize_section(message.randomness).width(Length::Fixed(500.0))
    ]
    .spacing(10)
}

fn probability_section<'a>(message: &ControlMessage) -> Row<'a, Message> {
    row![
        text("probability a cell triggers a note"),
        slider(0.0..=100.0, message.info.probability * 100.0, |x| {
            Message::ProbChanged(x / 100.0)
        }),
        text(format!("{}", message.info.probability)),
    ]
    .spacing(10)
}

fn randomize_section<'a>(r: f32) -> Row<'a, Message> {
    row![
        slider(0.0..=100.0, r * 100.0, |x| {
            Message::RandChanged(x / 100.0)
        }),
        text(format!("{r}")),
    ]
    .spacing(10)
}

fn map_buttons<'a>() -> Row<'a, Message> {
    row![
        button("randomize map")
            .on_press(Message::RandomizeMap)
            .style(theme::Button::Primary),
        button("randomize mask")
            .on_press(Message::RandomizeMask)
            .style(theme::Button::Primary),
    ]
    .spacing(10)
}

fn velocity_sliders<'a>(message: &ControlMessage) -> Column<'a, Message> {
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
    ]
    .spacing(10)
}

fn scale_selector<'a>(message: &ControlMessage) -> Row<'a, Message> {
    let scale = message.info.scale;
    let note = message.info.root.note;
    let accidental = message.info.root.accidental;
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
    .align_items(Alignment::Center)
}

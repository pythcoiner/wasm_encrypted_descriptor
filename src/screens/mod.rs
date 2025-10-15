use iced::{
    widget::{row, text, Button, Row, Space},
    Length, Theme,
};

use crate::message;

pub mod decrypt;
pub mod encrypt;
pub mod home;

pub const STR_LEN: usize = 30;

pub fn short_string(input: &str, len: usize) -> String {
    let init_len = input.chars().count();
    if init_len <= len * 2 + 3 {
        input.to_string()
    } else {
        let prefix: String = input.chars().take(len).collect();
        let suffix: String = input
            .chars()
            .rev()
            .take(len)
            .collect::<String>()
            .chars()
            .rev()
            .collect();
        format!("{}{}{}", prefix, "...", suffix)
    }
}

pub fn header(
    label: &'static str,
    msg: message::Navigation,
) -> Row<'static, message::Message, Theme, iced::Renderer> {
    row![
        Button::new("<").on_press(message::Message::Nav(msg)),
        Space::with_width(Length::Fill),
        text(label),
        Space::with_width(15),
        Space::with_width(Length::Fill),
    ]
}

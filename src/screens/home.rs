use iced::{
    widget::{Button, Column, Space},
    Command, Theme,
};

use crate::message::{self, Navigation};

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

#[derive(Default)]
pub struct Home {}

impl Home {
    pub fn route(msg: Message) -> message::Message {
        message::Message::Home(msg)
    }

    pub fn update(&mut self, _message: Message) -> Command<message::Message> {
        // NOTE: nothing to handle
        Command::none()
    }

    pub fn view(&self) -> Column<'_, message::Message, Theme, iced::Renderer> {
        let encrypt = Button::new("Encrypt").on_press(message::Message::Nav(Navigation::Encrypt));
        let decrypt = Button::new("Decrypt").on_press(message::Message::Nav(Navigation::Decrypt));

        Column::new()
            .push(Space::with_height(30))
            .push(encrypt)
            .push(decrypt)
            .align_items(iced::Alignment::Center)
            .spacing(10)
    }
}

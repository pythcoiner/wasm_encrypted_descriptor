pub mod message;
pub mod screens;

use iced::{
    executor,
    widget::{row, Space},
    Application, Command, Length, Settings, Theme,
};
use message::Message;
use screens::{decrypt::Decrypt, encrypt::Encrypt, home::Home};

pub fn main() -> iced::Result {
    let settings = Settings::default();
    Bed::run(settings)
}

#[derive(Default, Clone, Copy)]
pub enum Screen {
    #[default]
    Home,
    Encrypt,
    Decrypt,
}

#[derive(Default)]
struct Bed {
    screen: Screen,
    decrypt: Decrypt,
    encrypt: Encrypt,
    home: Home,
}

impl Application for Bed {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Bed::default(), Command::none())
    }

    fn title(&self) -> String {
        match self.screen {
            Screen::Home => "Bitcoin Encrypted Descriptor".into(),
            Screen::Encrypt => "Encrypt".into(),
            Screen::Decrypt => "Decrypt".into(),
        }
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Nav(navigation) => {
                match navigation {
                    message::Navigation::Home => self.screen = Screen::Home,
                    message::Navigation::Encrypt => self.screen = Screen::Encrypt,
                    message::Navigation::Decrypt => self.screen = Screen::Decrypt,
                }
                Command::none()
            }
            Message::Encrypt(message) => self.encrypt.update(message),
            Message::Decrypt(message) => self.decrypt.update(message),
            Message::Home(message) => self.home.update(message),
        }
    }

    fn view(&self) -> iced::Element<'_, Message, Theme, iced::Renderer> {
        let col = match self.screen {
            Screen::Home => self.home.view(),
            Screen::Encrypt => self.encrypt.view(),
            Screen::Decrypt => self.decrypt.view(),
        }
        .width(800);

        row![
            Space::with_width(Length::Fill),
            col,
            Space::with_width(Length::Fill),
        ]
        .into()
    }
}

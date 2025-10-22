use std::str::FromStr;

use bitcoin_encrypted_backup::{
    miniscript::{Descriptor, DescriptorPublicKey},
    EncryptedBackup,
};
use iced::{
    widget::{text, Button, Column, Container, Space},
    Command, Theme,
};

use crate::message::{self, Navigation};

use super::{header, short_string, STR_LEN};

#[derive(Debug, Clone)]
pub enum Message {
    LoadDescriptor,
    Encrypt,
    EncryptedPayload(Vec<u8>),
    Descriptor(Descriptor<DescriptorPublicKey>),
    Error(String),
    #[allow(dead_code)]
    None(String),
}

#[derive(Default)]
pub struct Encrypt {
    descriptor: Option<Descriptor<DescriptorPublicKey>>,
    encrypted_payload: Option<Vec<u8>>,
    error: Option<String>,
}

impl Encrypt {
    pub fn route(msg: Message) -> message::Message {
        message::Message::Encrypt(msg)
    }

    pub fn update(&mut self, message: Message) -> Command<message::Message> {
        match message {
            Message::LoadDescriptor => return self.on_load_descriptor(),
            Message::Descriptor(d) => self.on_update_descriptor(d),
            Message::Encrypt => return self.on_encrypt(),
            Message::EncryptedPayload(p) => return self.on_persist_backup(p),
            Message::Error(e) => self.on_error(e),
            Message::None(_) => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Column<'_, message::Message, Theme, iced::Renderer> {
        let header = header("Encrypt", Navigation::Home);

        let descriptor = if let Some(descr) = self.descriptor.as_ref() {
            Container::new(text(short_string(&descr.to_string(), STR_LEN).to_string()))
        } else {
            Container::new(
                Button::new("Load descriptor").on_press(Self::route(Message::LoadDescriptor)),
            )
        };

        let encrypt_btn = Button::new("Encrypt").on_press_maybe(
            self.descriptor
                .is_some()
                .then_some(Self::route(Message::Encrypt)),
        );

        let error = self.error.clone().map(|e| text(e.to_string()));

        Column::new()
            .push(Space::with_height(30))
            .push(header)
            .push(Space::with_height(30))
            .push(descriptor)
            .push(encrypt_btn)
            .push_maybe(error)
            .align_items(iced::Alignment::Center)
            .width(800)
            .spacing(10)
    }

    fn on_load_descriptor(&self) -> Command<message::Message> {
        Command::perform(
            async move {
                if let Some(fh) = rfd::AsyncFileDialog::new().pick_file().await {
                    let error = Self::route(Message::Error(
                        "File does not contains a valid descriptor.".into(),
                    ));
                    let bytes = fh.read().await;
                    let descr_str = match String::from_utf8(bytes) {
                        Ok(str) => str,
                        Err(_) => return error,
                    };
                    let descr_str = descr_str.trim().to_string();
                    let descr = match Descriptor::<DescriptorPublicKey>::from_str(&descr_str) {
                        Ok(d) => d,
                        Err(_) => return error,
                    };
                    Self::route(Message::Descriptor(descr))
                } else {
                    Self::route(Message::Error("Fail to open file".into()))
                }
            },
            |m| m,
        )
    }

    fn on_encrypt(&mut self) -> Command<message::Message> {
        if let Some(descriptor) = self.descriptor.clone() {
            Command::perform(
                async move {
                    let nonce = Self::generate_nonce();
                    let backp = EncryptedBackup::new()
                        .set_payload(&descriptor)
                        .expect("cannot fail")
                        .encrypt(nonce);
                    match backp {
                        Ok(bytes) => Self::route(Message::EncryptedPayload(bytes)),
                        Err(e) => Self::route(Message::Error(format!(
                            "Fail to encrypt descriptor: {e:?}"
                        ))),
                    }
                },
                |m| m,
            )
        } else {
            Command::none()
        }
    }

    fn on_update_descriptor(&mut self, descriptor: Descriptor<DescriptorPublicKey>) {
        self.descriptor = Some(descriptor.clone());
    }

    fn on_error(&mut self, error: String) {
        self.error = Some(error);
    }

    fn generate_nonce() -> [u8; 12] {
        let mut buf = [0u8; 12];
        getrandom::fill(&mut buf).expect("Failed to generate random bytes");
        buf
    }

    fn on_persist_backup(&mut self, bytes: Vec<u8>) -> Command<message::Message> {
        self.encrypted_payload = Some(bytes);
        self.error = Some("Descriptor encrypted!".into());
        self.on_persist()
    }

    fn on_persist(&mut self) -> Command<message::Message> {
        if let Some(payload) = self.encrypted_payload.clone() {
            return Command::perform(
                async move {
                    let fh = match rfd::AsyncFileDialog::new()
                        .set_file_name("descriptor.bed")
                        .save_file()
                        .await
                    {
                        Some(fh) => fh,
                        None => return Self::route(Message::Error("Fail to open file.".into())),
                    };
                    match fh.write(&payload).await {
                        Ok(_) => Self::route(Message::None(String::new())),
                        Err(_) => Self::route(Message::Error("Fail to write to disk.".into())),
                    }
                },
                |m| m,
            );
        }
        Command::none()
    }
}

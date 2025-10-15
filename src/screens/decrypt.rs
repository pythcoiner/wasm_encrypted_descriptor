use std::str::FromStr;

use bitcoin_encrypted_backup::{
    descriptor::dpk_to_pk, miniscript::DescriptorPublicKey, EncryptedBackup,
};
use iced::{
    widget::{text, Button, Column, Container, Space},
    Command, Theme,
};

use crate::message::{self, Navigation};

use super::{header, short_string, STR_LEN};

#[derive(Debug, Clone)]
pub enum Message {
    LoadBackup,
    LoadKey,
    Decrypt,
    EncryptedPayload(Vec<u8>),
    Key(DescriptorPublicKey),
    Descriptor(String),
    Persist,
    Error(String),
    #[allow(dead_code)]
    None(String),
}

#[derive(Default)]
pub struct Decrypt {
    key: Option<DescriptorPublicKey>,
    encrypted_payload: Option<Vec<u8>>,
    valid: bool,
    descriptor: Option<String>,
    error: Option<String>,
}

impl Decrypt {
    pub fn route(msg: Message) -> message::Message {
        message::Message::Decrypt(msg)
    }

    pub fn update(&mut self, message: Message) -> Command<message::Message> {
        match message {
            Message::LoadBackup => return self.on_load_backup(),
            Message::LoadKey => return self.on_load_key(),
            Message::Decrypt => return self.on_decrypt(),
            Message::EncryptedPayload(p) => self.on_update_encrypted_payload(p),
            Message::Key(key) => self.on_update_key(key),
            Message::Descriptor(d) => self.on_update_descriptor(d),
            Message::Persist => return self.on_persist(),
            Message::Error(e) => self.on_error(e),
            Message::None(_) => {}
        }
        Command::none()
    }

    pub fn view(&self) -> Column<'_, message::Message, Theme, iced::Renderer> {
        let header = header("Decrypt", Navigation::Home);
        let encrypted_payload = if self.encrypted_payload.is_some() {
            Container::new(text("[encrypted payload]"))
        } else {
            Container::new(
                Button::new("Load encrypted backup").on_press(Self::route(Message::LoadBackup)),
            )
        };

        let key = if let Some(key) = &self.key {
            Container::new(text(short_string(&key.to_string(), STR_LEN)))
        } else {
            Container::new(Button::new("Load key").on_press(Self::route(Message::LoadKey)))
        };

        let descriptor = (self.descriptor.clone())
            .map(|d| short_string(&d, STR_LEN))
            .map(text);

        let btn = match self.descriptor.is_some() {
            true => Button::new("Save").on_press(Self::route(Message::Persist)),
            false => Button::new("Decrypt")
                .on_press_maybe(self.valid.then_some(Self::route(Message::Decrypt))),
        };

        let error = self.error.clone().map(|e| text(e.to_string()));

        Column::new()
            .push(Space::with_height(30))
            .push(header)
            .push(Space::with_height(30))
            .push(encrypted_payload)
            .push(key)
            .push_maybe(descriptor)
            .push(btn)
            .push_maybe(error)
            .align_items(iced::Alignment::Center)
            .width(800)
            .spacing(10)
    }

    fn on_load_backup(&self) -> Command<message::Message> {
        Command::perform(
            async move {
                if let Some(fh) = rfd::AsyncFileDialog::new().pick_file().await {
                    let bytes = fh.read().await;
                    let backp = bitcoin_encrypted_backup::EncryptedBackup::new();
                    if backp.set_encrypted_payload(&bytes).is_ok() {
                        Self::route(Message::EncryptedPayload(bytes))
                    } else {
                        Self::route(Message::Error("Invalid payload".into()))
                    }
                } else {
                    Self::route(Message::Error("Fail to open file".into()))
                }
            },
            |m| m,
        )
    }

    fn on_load_key(&self) -> Command<message::Message> {
        Command::perform(
            async move {
                if let Some(fh) = rfd::AsyncFileDialog::new().pick_file().await {
                    let bytes = fh.read().await;
                    let error = Self::route(Message::Error(
                        "this file does not contains a valid key".into(),
                    ));
                    if let Ok(key_str) = String::from_utf8(bytes) {
                        if let Ok(key) = DescriptorPublicKey::from_str(key_str.trim()) {
                            Self::route(Message::Key(key))
                        } else {
                            error
                        }
                    } else {
                        error
                    }
                } else {
                    Self::route(Message::Error("Fail to open file".to_string()))
                }
            },
            |m| m,
        )
    }

    fn on_decrypt(&mut self) -> Command<message::Message> {
        if let (Some(payload), Some(key)) = (&self.encrypted_payload, &self.key) {
            let payload = payload.clone();
            let key = key.clone();
            Command::perform(
                async move {
                    let backp = EncryptedBackup::new()
                        .set_keys(vec![dpk_to_pk(&key)])
                        .set_encrypted_payload(&payload)
                        .expect("already checked");
                    if let Ok(decrypt) = backp.decrypt() {
                        match decrypt {
                            bitcoin_encrypted_backup::Decrypted::Descriptor(descriptor) => {
                                Self::route(Message::Descriptor(descriptor.to_string()))
                            }
                            _ => Self::route(Message::Error(
                                "Payload decrypted, but not of type descriptor".into(),
                            )),
                        }
                    } else {
                        Self::route(Message::Error(
                            "This key fails to decrypt the encrypted payload".into(),
                        ))
                    }
                },
                |m| m,
            )
        } else {
            Command::none()
        }
    }

    fn on_update_encrypted_payload(&mut self, bytes: Vec<u8>) {
        self.encrypted_payload = Some(bytes);
        self.check_decryptable();
    }

    fn on_update_key(&mut self, key: DescriptorPublicKey) {
        self.key = Some(key);
        self.check_decryptable();
    }

    fn on_update_descriptor(&mut self, descriptor: String) {
        self.descriptor = Some(descriptor);
    }

    fn on_error(&mut self, error: String) {
        self.error = Some(error);
    }

    fn on_persist(&mut self) -> Command<message::Message> {
        if let Some(descr) = self.descriptor.clone() {
            return Command::perform(
                async move {
                    let fh = match rfd::AsyncFileDialog::new()
                        .set_file_name("descriptor.txt")
                        .save_file()
                        .await
                    {
                        Some(fh) => fh,
                        None => return Self::route(Message::Error("Fail to open file.".into())),
                    };
                    match fh.write(descr.as_bytes()).await {
                        Ok(_) => Self::route(Message::None(String::new())),
                        Err(_) => Self::route(Message::Error("Fail to write to disk.".into())),
                    }
                },
                |m| m,
            );
        }
        Command::none()
    }

    fn check_decryptable(&mut self) {
        self.valid = self.key.is_some() && self.encrypted_payload.is_some();
    }
}

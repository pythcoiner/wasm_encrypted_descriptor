use crate::screens;

#[derive(Debug, Clone)]
pub enum Message {
    Nav(Navigation),
    Encrypt(screens::encrypt::Message),
    Decrypt(screens::decrypt::Message),
    Home(screens::home::Message),
}

#[derive(Debug, Clone)]
pub enum Navigation {
    Home,
    Encrypt,
    Decrypt,
}

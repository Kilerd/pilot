#[macro_use] extern crate log;



pub mod bot;
pub mod error;
pub mod method;
pub mod typing;


pub use crate::bot::Bot;
pub use crate::error::PilotError;

pub trait TelegramApiMethod: serde::Serialize {
    const METHOD: &'static str;
    type Response: serde::de::DeserializeOwned;

    fn get_method(&self) -> &'static str {
        Self::METHOD
    }
}
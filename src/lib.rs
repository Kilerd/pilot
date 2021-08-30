#[macro_use] extern crate log;

use crate::bot::Bot;
use crate::error::ApiResult;

pub mod bot;
pub mod error;
pub mod method;
pub mod typing;

pub trait TelegramApiMethod: serde::Serialize {
    const METHOD: &'static str;
    type Response: serde::de::DeserializeOwned;

    fn get_method(&self) -> &'static str {
        Self::METHOD
    }
}
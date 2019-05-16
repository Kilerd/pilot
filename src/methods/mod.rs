use crate::{bot::Bot, error::ApiResult};
use std::future::Future;

pub mod basic;

pub trait Method {
    type Type;

    fn send<F>(&self, bot: &Bot) -> F
    where
        F: Future<Output = ApiResult<Self::Type>>;
}

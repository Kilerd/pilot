use crate::{bot::Bot, error::ApiResult};
use std::future::Future as Future3;

pub mod basic;

pub trait Method {
    type Type;

    //    fn send(&self, bot: &Bot) -> impl Future3<Output = Result<ApiResult<Self::Type>, ()>> {
    //        bot.request("getMe", &self)
    //    }
}

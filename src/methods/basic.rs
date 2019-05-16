use crate::{
    bot::Bot,
    error::ApiResult,
    methods::Method,
    typing::{Chat, ChatType, Message, User},
};
use serde::{Deserialize, Serialize};
use std::future::Future;

#[derive(Deserialize, Serialize)]
pub struct GetMe {}

impl GetMe {
    fn send<F>(&self, bot: &Bot) -> impl Future<Output = Result<ApiResult<User>, ()>> {
        bot.request("getMe", &self)
    }
}
#[cfg(test)]
mod tests {
    use crate::{
        bot::Bot,
        methods::{basic::GetMe, Method},
    };
    use futures::executor;
    #[test]
    fn it_works() {
        let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap());
        let get_me = GetMe {};
        let x = get_me.send(&bot);

        executor::block_on(|| {
            let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap());
            let get_me = GetMe {};
            let x = get_me.send(&bot);
        });
    }
}

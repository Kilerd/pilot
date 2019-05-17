use crate::{
    bot::Bot,
    error::ApiResult,
    methods::Method,
    typing::{Chat, ChatType, Message, User},
};
use serde::{Deserialize, Serialize};
use futures01::Future as Future1;
use std::future::Future as Future3;


#[derive(Debug, Deserialize, Serialize)]
pub struct GetMe {}

impl GetMe {
    fn send(self, bot: &Bot) -> impl Future3<Output=Result<ApiResult<User>, ()>> {
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
    use crate::error::ApiResult;
    use crate::typing::User;
    use futures::compat::Future01CompatExt;
    use futures::future::{TryFutureExt, TryFuture, FutureExt};

    #[test]
    fn it_works() {

        let x = async {
            let bot = Bot::new(std::env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"));
            let get_me = GetMe {};
            let x = get_me.send(&bot);
            let a: Result<ApiResult<User>, ()> = x.await;
            println!("{:?}", a);
            assert_eq!(true, a.unwrap().ok);
            ()
        };


        tokio::run(x.unit_error().boxed().compat());
    }
}

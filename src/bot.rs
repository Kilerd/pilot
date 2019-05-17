use std::env;

use futures::{
    compat::Future01CompatExt,
    future::{Future as Future3, FutureExt, TryFuture, TryFutureExt},
    task::SpawnExt,
};
use reqwest::r#async::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::error::{ApiResult, BotResult};

use futures01::Future as Future1;
use std::fmt::Debug;

pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub fn request<P: Serialize + Debug, T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<String>,
        payload: &P,
    ) -> impl Future3<Output = Result<ApiResult<T>, ()>> {
        Client::new()
            .post(dbg!(format!(
                "https://api.telegram.org/bot{}/{}",
                env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"),
                method.into()
            )
            .as_str()))
            .json(dbg!(payload))
            .send()
            .and_then(|mut response: Response| {
                dbg!(response.status());
                response.json::<ApiResult<T>>()
            })
            .map_err(|e| {
                dbg!(e);
                ()
            })
            .compat()
    }
}

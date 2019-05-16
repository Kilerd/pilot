use std::env;

use futures::{
    compat::Future01CompatExt,
    future::{Future as Future3, FutureExt, TryFuture, TryFutureExt},
};
use reqwest::r#async::{Client, Response};
use serde::{Deserialize, Serialize};

use crate::error::{ApiResult, BotResult};

pub struct Bot {
    token: String,
}

impl Bot {
    pub fn new(token: impl Into<String>) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub fn request<P: Serialize, T: serde::de::DeserializeOwned>(
        &self,
        method: impl Into<String>,
        payload: P,
    ) -> impl Future3<Output = Result<ApiResult<T>, ()>> {
        Client::new()
            .post(
                format!(
                    "https://api.telegram.org/bot{}/{}",
                    env::var("BOT_TOKEN").expect("need to set BOT_TOKEN as environment variable"),
                    method.into()
                )
                .as_str(),
            )
            .json(&payload)
            .send()
            .compat()
            .and_then(|mut response: Response| response.json::<ApiResult<T>>())
            .map_err(|_| ())
    }
}

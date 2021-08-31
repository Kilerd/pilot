use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{ApiResult, BotResult, PilotError};
use crate::method::webhook::GetUpdates;
use crate::typing::{UpdateMessage};
use crate::TelegramApiMethod;
use tokio::time::Duration;

pub struct Bot {
    secret_key: String,
    commands: HashMap<String, Vec<Box<dyn Fn(Arc<Bot>, Arc<UpdateMessage>) + Send + Sync>>>,
    other: Option<Box<dyn Fn(Arc<Bot>, Arc<UpdateMessage>) + Send + Sync>>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            secret_key: std::env::var("TELEGRAM_BOT_SECRET_KEY")
                .expect("need to set TELEGRAM_BOT_SECRET_KEY as environment variable"),
            commands: Default::default(),
            other: None,
        }
    }

    pub async fn request<T: TelegramApiMethod>(
        &self,
        method: T,
    ) -> Result<BotResult<T::Response>, PilotError> {
        let uri = format!(
            "https://api.telegram.org/bot{}/{}",
            self.secret_key,
            method.get_method()
        );
        let client = reqwest::Client::new();
        let bot_result = client
            .post(uri)
            .json(&method)
            .send()
            .await?
            .json::<ApiResult<T::Response>>()
            .await?
            .into_result();
        Ok(bot_result)
    }

    pub fn command<H, F>(&mut self, command: &str, handler: H)
    where
        H: (Fn(Arc<Bot>, Arc<UpdateMessage>) -> F) + Send + Sync + 'static,
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.commands
            .entry(command.to_uppercase())
            .or_insert(Vec::new())
            .push(Box::new(move |bot, update| {
                tokio::spawn(handler(bot, update));
            }));
    }
    pub fn other<H, F>(&mut self, handler: H)
    where
        H: (Fn(Arc<Bot>, Arc<UpdateMessage>) -> F) + Send + Sync + 'static,
        F: std::future::Future<Output = ()> + Send + 'static,
    {
        self.other = Some(Box::new(move |bot, update| {
            tokio::spawn(handler(bot, update));
        }));
    }

    pub async fn polling(self) {
        let arc_self = Arc::new(self);

        let mut interval = tokio::time::interval(Duration::from_millis(100));
        let mut offset = 0;
        loop {
            interval.tick().await;
            debug!("requesting updates with offset: {}", &offset);
            let result = arc_self
                .request(GetUpdates {
                    offset: if offset == 0 { None } else { Some(offset + 1) },
                    limit: None,
                    timeout: Some(30),
                    allowed_updates: None,
                })
                .await;
            let bot_result = match result {
                Ok(br) => br,
                Err(e) => {
                    warn!("fail on polling: {}", e);
                    continue;
                }
            };
            match bot_result {
                Ok(updates) => {
                    for one_msg in updates {
                        offset = one_msg.update_id;
                        let option1 = match &one_msg.message {
                            UpdateMessage::Message(msg) | UpdateMessage::EditedMessage(msg) => {
                                msg.text.as_ref()
                            }
                            _ => None,
                        };
                        let option2 = option1
                            .and_then(|text| {
                                if text.starts_with('/') {
                                    let n: Vec<&str> = text.splitn(2, ' ').collect();
                                    let command = &n[0][1..n[0].len()];
                                    Some(command)
                                } else {
                                    None
                                }
                            })
                            .map(|command| command.to_uppercase());

                        let msg = Arc::new(one_msg.message);

                        if let Some(command) = option2 {
                            debug!("match command {}", &command);
                            let option = arc_self.commands.get(&command);
                            if let Some(handlers) = option {
                                for one_handler in handlers {
                                    one_handler(arc_self.clone(), msg.clone());
                                }
                            }
                        } else {
                            debug!("command does not match, dispatch it to other handlers.");
                            if let Some(other_handler) = arc_self.other.as_ref() {
                                other_handler(arc_self.clone(), msg.clone());
                            }
                        }
                    }
                }
                Err(e) => {
                    warn!("fail on fetching telegram updates: {:?}", e)
                }
            }
        }
    }
}

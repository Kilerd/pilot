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

#[cfg(test)]
mod test {
    use crate::bot::Bot;
    use crate::method::send::SendMessage;
    use crate::typing::UpdateMessage;

    #[tokio::test]
    async fn test_all() {
        let mut bot = Bot::new();
        bot.command("ping", |bot, msg| async move {
            match msg.as_ref() {
                UpdateMessage::Message(msg) => {
                    let message = SendMessage::new(msg.chat.id.to_string(), "pong");
                    bot.request(message).await;
                }
                _ => {}
            }
        });
        bot.polling().await;
    }
}

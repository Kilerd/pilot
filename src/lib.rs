use crate::bot::Bot;
use crate::error::ApiResult;


pub mod bot;
pub mod error;
pub mod method;
pub mod typing;

pub trait TelegramApiMethod: serde::Serialize {
    const METHOD : &'static str;
    type Response: serde::de::DeserializeOwned;

    fn get_method(&self) -> &'static str {
        Self::METHOD
    }
}



#[cfg(test)]
mod test {
    use crate::bot::Bot;

    #[tokio::test]
    async fn test_all() {
        let mut bot = Bot::new();
        bot.command("HELP", |bot, msg| async {
            dbg!(msg);
            println!("hello");
        });
        bot.run().await;
    }
}
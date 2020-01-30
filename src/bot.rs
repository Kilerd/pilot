use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use crate::error::{ApiResult, BotResult};
use crate::TelegramApiMethod;
use crate::typing::Update;

pub struct Bot {
    pub secret_key: String,
    pub commands: HashMap<String, Vec<Box<Arc<dyn Fn(Arc<Bot>, Arc<Update>) + Send + Sync>>>>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            secret_key: std::env::var("TELEGRAM_BOT_SECRET_KEY")
                .expect("need to set TELEGRAM_BOT_SECRET_KEY as environment variable"),
            commands: Default::default(),
        }
    }

    pub async fn request<T: TelegramApiMethod>(&self, method: T) -> BotResult<T::Response> {
        let result = surf::post(format!("https://api.telegram.org/bot{}/{}", self.secret_key, method.get_method()))
            .body_json(&method)
            .unwrap().await;
        let x = result.unwrap().body_json::<ApiResult<T::Response>>().await;
        x.unwrap().into_result()
    }

    pub fn command<H, F>(&mut self, command: &str, handler: H) where
        H: (Fn(Arc<Bot>, Arc<Update>) -> F) + Send + Sync + 'static,
        F: std::future::Future<Output=()> + Send + 'static
    {
        self.commands
            .entry(command.to_string())
            .or_insert(Vec::new())
            .push(Box::new(Arc::new(move |bot, update| {
                tokio::spawn(handler(bot, update));
            })));
    }
    pub async fn handle(&self, arc: Arc<Bot>,  update: Update) {
        let option = self.commands.get("ping");
        let arc1 = Arc::new(update);
        let x = option.unwrap();
        for handler in x {
            handler(arc.clone(), arc1.clone())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::bot::Bot;
    use crate::method::GetMe;
    use crate::typing::{Update, UpdateMessage};
    use std::sync::Arc;
//
//    #[tokio::test]
//    async fn test_binding_server() {
//        let bot = Bot::new();
//        let result = bot.request(GetMe {}).await;
//        dbg!(result);
//    }
//
//    #[tokio::test]
//    async fn handle_a_new_command() {
//        let mut bot = Bot::new();
//        let x = |bbot:Arc<Bot>, update:Arc<Update>| {
//            async move {
//                println!("pong");
//                let result = bbot.request(GetMe {}).await;
//                dbg!(result);
//            }
//        };
//        bot.command("ping", x);
//        let update1 = Update { update_id: 0, message: UpdateMessage::Unknown };
//        let arc = Arc::new(bot);
//
//        bot.handle(arc, update1).await;
//    }
}
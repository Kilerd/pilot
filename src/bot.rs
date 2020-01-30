use crate::TelegramApiMethod;
use crate::error::{BotResult, ApiResult};

pub struct Bot {
    pub secret_key: String,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            secret_key: std::env::var("TELEGRAM_BOT_SECRET_KEY")
                .expect("need to set TELEGRAM_BOT_SECRET_KEY as environment variable"),
        }
    }

    pub async fn request<T: TelegramApiMethod>(&self, method: T) -> BotResult<T::Response> {
        let result = surf::post(format!("https://api.telegram.org/bot{}/{}", self.secret_key, method.get_method()))
            .body_json(&method)
            .unwrap().await;
        let x = result.unwrap().body_json::<ApiResult<T::Response>>().await;
        x.unwrap().into_result()
    }
}

#[cfg(test)]
mod test {
    use crate::bot::Bot;
    use crate::method::GetMe;
    use tokio::runtime::Runtime;

    #[test]
     fn test_binding_server() {
        let mut result1 = Runtime::new().unwrap();
        result1.block_on(async {
            let bot = Bot::new();
            let result = bot.request(GetMe {}).await;
            dbg!(result);
        })
    }
}

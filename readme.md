# Pilot
the pilot manipulating telegram bot

this project is highly WIP, and the name would also be renamed in the later release, please be really deep considering when trying it. 

## Get started

```rust
use pilot::{Bot, UpdateMessage, SendMessage};
async fn main() {
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
```
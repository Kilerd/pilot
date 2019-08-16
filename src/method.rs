use crate::typing::{
    ForceReply, InlineKeyboardButton, ParseMode, ReplyKeyboardMarkup, ReplyKeyboardRemove,
};
use crate::{
    bot::Bot,
    error::ApiResult,
    typing::{CallbackQuery, Location, Message, OrderInfo, Poll, ShippingAddress, User},
};
use actix::Handler;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::borrow::Cow;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMe {}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMessage<'a> {
    pub chat_id: Cow<'a, str>,
    pub text: Cow<'a, str>,
    pub parse_mode: Option<ParseMode>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'a> SendMessage<'a> {
    pub fn new(chat_id: impl Into<Cow<'a, str>>, text: impl Into<Cow<'a, str>>) -> SendMessage<'a> {
        SendMessage {
            chat_id: chat_id.into(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }
    pub fn parse_mode(self, mode: ParseMode) -> SendMessage<'a> {
        Self {
            parse_mode: Some(mode),
            ..self
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(Vec<Vec<InlineKeyboardButton>>),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardMessage {
    pub chat_id: String,
    pub from_chat_id: String,
    pub disable_notification: Option<bool>,
    pub message_id: i32,
}

impl actix::Message for GetMe {
    type Result = ();
}

impl<'a> actix::Message for SendMessage<'a> {
    type Result = ();
}

impl Handler<GetMe> for Bot {
    type Result = ();

    fn handle(&mut self, msg: GetMe, ctx: &mut Self::Context) -> Self::Result {
        reqwest::Client::new()
            .post(format!("https://api.telegram.org/bot{}/GetMe", self.secret_key).as_str())
            .json(&msg)
            .send()
            .map_err(|e| {
                dbg!(&e);
                e
            });

        ()
    }
}

impl<'a> Handler<SendMessage<'a>> for Bot {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) -> Self::Result {
        println!("doing sendmessage");
        reqwest::Client::new()
            .post(
                format!(
                    "https://api.telegram.org/bot{}/sendMessage",
                    self.secret_key
                )
                .as_str(),
            )
            .json(&msg)
            .send()
            .map_err(|e| {
                dbg!(&e);
                e
            });

        ()
    }
}

/// The method for receiving incoming updates using long polling
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUpdates {
    /// Identifier of the first update to be returned. Must be greater by one than the highest among the identifiers of previously received updates. By default, updates starting with the earliest unconfirmed update are returned. An update is considered confirmed as soon as getUpdates is called with an offset higher than its update_id. The negative offset can be specified to retrieve updates starting from -offset update from the end of the updates queue. All previous updates will forgotten.
    pub offset: Option<i32>,
    /// Limits the number of updates to be retrieved. Values between 1—100 are accepted. Defaults to 100.
    pub limit: Option<i32>,
    /// Timeout in seconds for long polling. Defaults to 0, i.e. usual short polling. Should be positive, short polling should be used for testing purposes only.
    pub timeout: Option<i32>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    ///
    ///Please note that this parameter doesn't affect updates created before the call to the getUpdates, so unwanted updates may be received for a short period of time.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to specify a url and receive incoming updates via an outgoing webhook. Whenever there is an update for the bot, we will send an HTTPS POST request to the specified url, containing a JSON-serialized Update. In case of an unsuccessful request, we will give up after a reasonable amount of attempts. Returns True on success.
///
/// If you'd like to make sure that the Webhook request comes from Telegram, we recommend using a secret path in the URL, e.g. https://www.example.com/<token>. Since nobody else knows your bot‘s token, you can be pretty sure it’s us.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SetWebhook {
    /// HTTPS url to send updates to. Use an empty string to remove webhook integration
    pub url: String,
    /// Upload your public key certificate so that the root certificate in use can be checked. See our self-signed guide for details.
    pub certificate: Option<String>,
    /// Maximum allowed number of simultaneous HTTPS connections to the webhook for update delivery, 1-100. Defaults to 40. Use lower values to limit the load on your bot‘s server, and higher values to increase your bot’s throughput.
    pub max_connections: Option<i32>,
    /// List the types of updates you want your bot to receive. For example, specify [“message”, “edited_channel_post”, “callback_query”] to only receive updates of these types. See Update for a complete list of available update types. Specify an empty list to receive all updates regardless of type (default). If not specified, the previous setting will be used.
    pub allowed_updates: Option<Vec<String>>,
}

/// Use this method to remove webhook integration if you decide to switch back to getUpdates. Returns True on success.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteWebhook;

/// Use this method to get current webhook status. Requires no parameters. On success, returns a WebhookInfo object. If the bot is using getUpdates, will return an object with the url field empty.
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetWebhookInfo;

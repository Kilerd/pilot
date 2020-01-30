use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::typing::{InlineKeyboardMarkup, ParseMode};

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct AnswerInlineQuery {
    inline_query_id: String,
    results: Vec<InlineQueryResult>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag="type", rename_all = "snake_case")]
enum InlineQueryResult {
    Article(InlineQueryResultArticle),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
struct InlineQueryResultArticle {
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    reply_markup: Option<InlineKeyboardMarkup>,
    url: Option<String>,
    hide_url: Option<bool>,
    description: Option<String>,
    thumb_url: Option<String>,
    thumb_width: Option<i32>,
    thumb_height: Option<i32>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
struct InputTextMessageContent {
    message_text: String,
    parse_mode: Option<ParseMode>,
    disable_web_page_preview: Option<bool>,
}
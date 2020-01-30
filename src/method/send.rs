use std::borrow::Cow;
use crate::typing::{ParseMode, ReplyMarkup};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
    pub fn disable_web_page_preview(self, value: bool) -> SendMessage<'a> {
        Self {
            disable_web_page_preview: Some(value),
            ..self
        }
    }
    pub fn disable_notification(self, value: bool) -> SendMessage<'a> {
        Self {
            disable_notification: Some(value),
            ..self
        }
    }
    pub fn reply(self, to: i32) -> SendMessage<'a> {
        Self {
            reply_to_message_id: Some(to),
            ..self
        }
    }
}



#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ForwardMessage<'a> {
    pub chat_id: Cow<'a, str>,
    pub from_chat_id: Cow<'a, str>,
    pub disable_notification: Option<bool>,
    pub message_id: i32,
}

impl<'a> ForwardMessage<'a> {
    pub fn new(message_id: i32, from: impl Into<Cow<'a, str>>, to: impl Into<Cow<'a, str>>) -> ForwardMessage<'a> {
        Self {
            chat_id: to.into(),
            from_chat_id: from.into(),
            disable_notification: None,
            message_id
        }
    }

    pub fn disable_notification(self) -> ForwardMessage<'a> {
        Self {
            disable_notification: Some(true),
            ..self
        }
    }
    pub fn enable_notification(self) -> ForwardMessage<'a> {
        Self {
            disable_notification: None,
            ..self
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum InputFile {
    FileId(String),
    URL(String),
    // todo input file
    File()
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPhoto<'a> {
    pub chat_id: Cow<'a, str>,
    pub photo: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendAudio<'a> {
    pub chat_id: Cow<'a, str>,
    pub audio: InputFile,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub duration: Option<i32>,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub thumb: Option<InputFile>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendDocument<'a> {
    pub chat_id: Cow<'a, str>,
    pub document: InputFile,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}


#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendVideo<'a> {
    pub chat_id: Cow<'a, str>,
    pub video: InputFile,
    pub duration: Option<i32>,
    pub width:Option<i32>,
    pub height: Option<i32>,
    pub thumb: Option<InputFile>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub supports_streaming: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i32>,
    pub reply_markup: Option<ReplyMarkup>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendAnimation {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendVoice {
// todo
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendVideoNote {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendMediaGroup {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendLocation {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EditMessageLiveLocation {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StopMessageLiveLocation {
// todo
}
#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendVenue {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendContact {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendPoll {
// todo
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SendChatAction {
// todo
}

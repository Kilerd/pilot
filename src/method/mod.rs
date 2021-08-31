use crate::typing::*;
use serde::{Deserialize, Serialize};
use crate::TelegramApiMethod;


pub mod webhook;
pub mod send;
pub mod info;
pub mod inline;
pub mod updating;

pub use webhook::*;
pub use send::*;
pub use info::*;
pub use inline::*;
pub use updating::*;


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetMe {}


macro_rules! impl_api_method {
    ($($name: ty : $method_name: tt -> $ret: ty),*) => {
     $(

        impl TelegramApiMethod for $name {
            const METHOD: &'static str = $method_name;
            type Response = $ret;
        }
     )*
    };
}

#[rustfmt::skip]
impl_api_method!(
    GetMe:                      "GetMe" ->                      User,

    GetUpdates:                 "GetUpdates" ->                 Vec<Update>,
    GetWebhookInfo:             "GetWebhookInfo" ->             WebhookInfo,
    SetWebhook:                 "SetWebhook" ->                 bool,
    DeleteWebhook:              "deleteWebhook" ->              bool,

    SendMessage<'_>:            "SendMessage" ->                Message,
    ForwardMessage<'_>:         "ForwardMessage"->              Message,
    SendPhoto<'_>:              "SendPhoto" ->                  Message,
    SendAudio<'_>:              "SendAudio" ->                  Message,
    SendDocument<'_>:           "SendDocument" ->               Message,
    SendVideo<'_>:              "SendVideo" ->                  Message,
    SendAnimation:              "SendAnimation" ->              Message,
    SendVoice:                  "SendVoice" ->                  Message,
    SendVideoNote:              "SendVideoNote" ->              Message,
    SendMediaGroup:             "SendMediaGroup" ->             Message,
    SendLocation:               "SendLocation" ->               Message,
    // todo either Message or bool
    EditMessageLiveLocation:    "EditMessageLiveLocation" ->    Message,
    StopMessageLiveLocation:    "StopMessageLiveLocation" ->    Message,
    SendVenue:                  "SendVenue" ->                  Message,
    SendContact:                "SendContact" ->                Message,
    SendPoll:                   "SendPoll" ->                   Message,
    SendChatAction:             "SendChatAction" ->             bool,


    GetUserProfilePhotos:       "GetUserProfilePhotos" ->       UserProfilePhotos,
    GetFile:                    "GetFile" ->                    File,

    DeleteMessage<'_>:              "deleteMessage" ->              bool
);


#[cfg(test)]
mod test {
    use crate::method::GetMe;

    #[test]
    fn serde_get_me() {
        let me = GetMe {};
        let result = serde_json::to_string(&me).unwrap();

        let expected = r#"{}"#;
        assert_eq!(result, expected);
    }
}
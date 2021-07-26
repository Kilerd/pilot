use std::borrow::Cow;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DeleteMessage<'a> {
    pub chat_id: Cow<'a, str>,
    pub message_id: i32,
}
